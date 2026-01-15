use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    BlogCategory, BlogPost, BlogPostStatus, CreateBlogPostRequest, UpdateBlogPostRequest,
};

fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub async fn create_blog_post(
    pool: &SqlitePool,
    req: &CreateBlogPostRequest,
    author_id: &str,
) -> Result<BlogPost, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let slug = req
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&req.title));

    // Check if slug already exists
    let existing = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(pool)
        .await?;

    let final_slug = if existing.is_some() {
        format!("{}-{}", slug, &id[..8])
    } else {
        slug
    };

    let category = req
        .category
        .as_ref()
        .unwrap_or(&BlogCategory::News)
        .to_string();

    let tags = req
        .tags
        .as_ref()
        .map(|t| t.join(","))
        .unwrap_or_default();

    let status = if req.publish_immediately.unwrap_or(false) {
        BlogPostStatus::Published.to_string()
    } else {
        BlogPostStatus::Draft.to_string()
    };

    let publish_date = if req.publish_immediately.unwrap_or(false) {
        Some(now.clone())
    } else {
        None
    };

    sqlx::query(
        r#"
        INSERT INTO blog_posts (
            id, title, slug, summary, content, category, tags,
            featured_image_url, author_id, status, publish_date,
            is_featured, view_count, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.title)
    .bind(&final_slug)
    .bind(&req.summary)
    .bind(&req.content)
    .bind(&category)
    .bind(&tags)
    .bind(&req.featured_image_url)
    .bind(author_id)
    .bind(&status)
    .bind(&publish_date)
    .bind(req.is_featured.unwrap_or(false))
    .bind(0)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_blog_post_by_id(pool, &id).await
}

pub async fn get_blog_post_by_id(pool: &SqlitePool, id: &str) -> Result<BlogPost, AppError> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))
}

pub async fn get_blog_post_by_slug(pool: &SqlitePool, slug: &str) -> Result<BlogPost, AppError> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))
}

pub async fn update_blog_post(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateBlogPostRequest,
) -> Result<BlogPost, AppError> {
    let existing = get_blog_post_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let title = req.title.as_ref().unwrap_or(&existing.title);
    let slug = req.slug.clone().unwrap_or(existing.slug);
    let summary = req.summary.clone().or(existing.summary);
    let content = req.content.as_ref().unwrap_or(&existing.content);
    let category = req
        .category
        .as_ref()
        .map(|c| c.to_string())
        .unwrap_or(existing.category);
    let tags = req
        .tags
        .as_ref()
        .map(|t| t.join(","))
        .or(existing.tags);
    let featured_image_url = req.featured_image_url.clone().or(existing.featured_image_url);
    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status.clone());
    let is_featured = req.is_featured.unwrap_or(existing.is_featured);

    // Auto-set publish_date when status changes to published
    let publish_date = if status == "published" && existing.status != "published" {
        Some(now.clone())
    } else {
        existing.publish_date
    };

    sqlx::query(
        r#"
        UPDATE blog_posts SET
            title = ?,
            slug = ?,
            summary = ?,
            content = ?,
            category = ?,
            tags = ?,
            featured_image_url = ?,
            status = ?,
            publish_date = ?,
            is_featured = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(title)
    .bind(&slug)
    .bind(&summary)
    .bind(content)
    .bind(&category)
    .bind(&tags)
    .bind(&featured_image_url)
    .bind(&status)
    .bind(&publish_date)
    .bind(is_featured)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_blog_post_by_id(pool, id).await
}

pub async fn delete_blog_post(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM blog_posts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Blog post not found".to_string()));
    }

    Ok(())
}

pub async fn list_blog_posts(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    status: Option<&str>,
    category: Option<&str>,
    tag: Option<&str>,
    featured_only: bool,
    author_id: Option<&str>,
) -> Result<(Vec<BlogPost>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM blog_posts WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM blog_posts WHERE 1=1".to_string();

    if let Some(s) = status {
        count_query.push_str(&format!(" AND status = '{}'", s));
        select_query.push_str(&format!(" AND status = '{}'", s));
    }

    if let Some(c) = category {
        count_query.push_str(&format!(" AND category = '{}'", c));
        select_query.push_str(&format!(" AND category = '{}'", c));
    }

    if let Some(t) = tag {
        count_query.push_str(&format!(" AND tags LIKE '%{}%'", t));
        select_query.push_str(&format!(" AND tags LIKE '%{}%'", t));
    }

    if featured_only {
        count_query.push_str(" AND is_featured = 1");
        select_query.push_str(" AND is_featured = 1");
    }

    if let Some(aid) = author_id {
        count_query.push_str(&format!(" AND author_id = '{}'", aid));
        select_query.push_str(&format!(" AND author_id = '{}'", aid));
    }

    select_query.push_str(" ORDER BY is_featured DESC, publish_date DESC, created_at DESC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let posts = sqlx::query_as::<_, BlogPost>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((posts, total.0))
}

pub async fn list_published_blog_posts(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    category: Option<&str>,
    tag: Option<&str>,
) -> Result<(Vec<BlogPost>, i64), AppError> {
    list_blog_posts(pool, offset, limit, Some("published"), category, tag, false, None).await
}

pub async fn increment_view_count(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE blog_posts SET view_count = view_count + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn publish_blog_post(pool: &SqlitePool, id: &str) -> Result<BlogPost, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE blog_posts SET status = 'published', publish_date = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&now)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_blog_post_by_id(pool, id).await
}

pub async fn archive_blog_post(pool: &SqlitePool, id: &str) -> Result<BlogPost, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query("UPDATE blog_posts SET status = 'archived', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_blog_post_by_id(pool, id).await
}

pub async fn get_featured_posts(pool: &SqlitePool, limit: i64) -> Result<Vec<BlogPost>, AppError> {
    let posts = sqlx::query_as::<_, BlogPost>(
        "SELECT * FROM blog_posts WHERE status = 'published' AND is_featured = 1 ORDER BY publish_date DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(posts)
}

pub async fn get_recent_posts(pool: &SqlitePool, limit: i64) -> Result<Vec<BlogPost>, AppError> {
    let posts = sqlx::query_as::<_, BlogPost>(
        "SELECT * FROM blog_posts WHERE status = 'published' ORDER BY publish_date DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(posts)
}
