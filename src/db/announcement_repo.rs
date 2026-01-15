use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Announcement, AnnouncementPriority, AnnouncementStatus, CreateAnnouncementRequest,
    TargetAudience, UpdateAnnouncementRequest,
};

pub async fn create_announcement(
    pool: &SqlitePool,
    req: &CreateAnnouncementRequest,
    author_id: &str,
) -> Result<Announcement, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let priority = req
        .priority
        .as_ref()
        .unwrap_or(&AnnouncementPriority::Normal)
        .to_string();

    let target_audience = req
        .target_audience
        .as_ref()
        .unwrap_or(&TargetAudience::All)
        .to_string();

    let status = if req.publish_immediately.unwrap_or(false) {
        AnnouncementStatus::Published.to_string()
    } else {
        AnnouncementStatus::Draft.to_string()
    };

    let publish_date = if req.publish_immediately.unwrap_or(false) {
        Some(now.clone())
    } else {
        req.publish_date.clone()
    };

    sqlx::query(
        r#"
        INSERT INTO announcements (
            id, title, content, priority, status, target_audience,
            author_id, publish_date, expiry_date, is_pinned, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&priority)
    .bind(&status)
    .bind(&target_audience)
    .bind(author_id)
    .bind(&publish_date)
    .bind(&req.expiry_date)
    .bind(req.is_pinned.unwrap_or(false))
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_announcement_by_id(pool, &id).await
}

pub async fn get_announcement_by_id(pool: &SqlitePool, id: &str) -> Result<Announcement, AppError> {
    sqlx::query_as::<_, Announcement>("SELECT * FROM announcements WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Announcement not found".to_string()))
}

pub async fn update_announcement(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateAnnouncementRequest,
) -> Result<Announcement, AppError> {
    let existing = get_announcement_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let title = req.title.as_ref().unwrap_or(&existing.title);
    let content = req.content.as_ref().unwrap_or(&existing.content);
    let priority = req
        .priority
        .as_ref()
        .map(|p| p.to_string())
        .unwrap_or(existing.priority);
    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status.clone());
    let target_audience = req
        .target_audience
        .as_ref()
        .map(|t| t.to_string())
        .unwrap_or(existing.target_audience);
    let publish_date = req.publish_date.clone().or(existing.publish_date);
    let expiry_date = req.expiry_date.clone().or(existing.expiry_date);
    let is_pinned = req.is_pinned.unwrap_or(existing.is_pinned);

    // Auto-set publish_date when status changes to published
    let publish_date = if status == "published" && existing.status != "published" {
        Some(now.clone())
    } else {
        publish_date
    };

    sqlx::query(
        r#"
        UPDATE announcements SET
            title = ?,
            content = ?,
            priority = ?,
            status = ?,
            target_audience = ?,
            publish_date = ?,
            expiry_date = ?,
            is_pinned = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(&priority)
    .bind(&status)
    .bind(&target_audience)
    .bind(&publish_date)
    .bind(&expiry_date)
    .bind(is_pinned)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_announcement_by_id(pool, id).await
}

pub async fn delete_announcement(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM announcements WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Announcement not found".to_string()));
    }

    Ok(())
}

pub async fn list_announcements(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    status: Option<&str>,
    priority: Option<&str>,
    target_audience: Option<&str>,
    pinned_only: bool,
) -> Result<(Vec<Announcement>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM announcements WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM announcements WHERE 1=1".to_string();

    if let Some(s) = status {
        count_query.push_str(&format!(" AND status = '{}'", s));
        select_query.push_str(&format!(" AND status = '{}'", s));
    }

    if let Some(p) = priority {
        count_query.push_str(&format!(" AND priority = '{}'", p));
        select_query.push_str(&format!(" AND priority = '{}'", p));
    }

    if let Some(ta) = target_audience {
        count_query.push_str(&format!(" AND target_audience = '{}'", ta));
        select_query.push_str(&format!(" AND target_audience = '{}'", ta));
    }

    if pinned_only {
        count_query.push_str(" AND is_pinned = 1");
        select_query.push_str(" AND is_pinned = 1");
    }

    select_query.push_str(" ORDER BY is_pinned DESC, priority DESC, created_at DESC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let announcements = sqlx::query_as::<_, Announcement>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((announcements, total.0))
}

pub async fn list_active_announcements(
    pool: &SqlitePool,
    target_audience: Option<&str>,
) -> Result<Vec<Announcement>, AppError> {
    let now = Utc::now().to_rfc3339();
    let mut query = format!(
        "SELECT * FROM announcements WHERE status = 'published' AND (expiry_date IS NULL OR expiry_date > '{}')",
        now
    );

    if let Some(ta) = target_audience {
        query.push_str(&format!(" AND (target_audience = '{}' OR target_audience = 'all')", ta));
    }

    query.push_str(" ORDER BY is_pinned DESC, priority DESC, publish_date DESC");

    let announcements = sqlx::query_as::<_, Announcement>(&query)
        .fetch_all(pool)
        .await?;

    Ok(announcements)
}

pub async fn publish_announcement(pool: &SqlitePool, id: &str) -> Result<Announcement, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE announcements SET status = 'published', publish_date = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&now)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_announcement_by_id(pool, id).await
}

pub async fn archive_announcement(pool: &SqlitePool, id: &str) -> Result<Announcement, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query("UPDATE announcements SET status = 'archived', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_announcement_by_id(pool, id).await
}
