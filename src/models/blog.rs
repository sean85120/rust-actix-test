use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlogPostStatus {
    Draft,
    Published,
    Archived,
}

impl Default for BlogPostStatus {
    fn default() -> Self {
        BlogPostStatus::Draft
    }
}

impl From<String> for BlogPostStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "published" => BlogPostStatus::Published,
            "archived" => BlogPostStatus::Archived,
            _ => BlogPostStatus::Draft,
        }
    }
}

impl ToString for BlogPostStatus {
    fn to_string(&self) -> String {
        match self {
            BlogPostStatus::Draft => "draft".to_string(),
            BlogPostStatus::Published => "published".to_string(),
            BlogPostStatus::Archived => "archived".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlogCategory {
    News,
    Training,
    Nutrition,
    Events,
    Tips,
    Stories,
    Announcements,
    Other,
}

impl Default for BlogCategory {
    fn default() -> Self {
        BlogCategory::News
    }
}

impl From<String> for BlogCategory {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "training" => BlogCategory::Training,
            "nutrition" => BlogCategory::Nutrition,
            "events" => BlogCategory::Events,
            "tips" => BlogCategory::Tips,
            "stories" => BlogCategory::Stories,
            "announcements" => BlogCategory::Announcements,
            "other" => BlogCategory::Other,
            _ => BlogCategory::News,
        }
    }
}

impl ToString for BlogCategory {
    fn to_string(&self) -> String {
        match self {
            BlogCategory::News => "news".to_string(),
            BlogCategory::Training => "training".to_string(),
            BlogCategory::Nutrition => "nutrition".to_string(),
            BlogCategory::Events => "events".to_string(),
            BlogCategory::Tips => "tips".to_string(),
            BlogCategory::Stories => "stories".to_string(),
            BlogCategory::Announcements => "announcements".to_string(),
            BlogCategory::Other => "other".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub category: String,
    pub tags: Option<String>,
    pub featured_image_url: Option<String>,
    pub author_id: String,
    pub status: String,
    pub publish_date: Option<String>,
    pub is_featured: bool,
    pub view_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub category: BlogCategory,
    pub tags: Vec<String>,
    pub featured_image_url: Option<String>,
    pub author_id: String,
    pub status: BlogPostStatus,
    pub publish_date: Option<String>,
    pub is_featured: bool,
    pub view_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<BlogPost> for BlogPostResponse {
    fn from(b: BlogPost) -> Self {
        let tags: Vec<String> = b
            .tags
            .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        BlogPostResponse {
            id: b.id,
            title: b.title,
            slug: b.slug,
            summary: b.summary,
            content: b.content,
            category: BlogCategory::from(b.category),
            tags,
            featured_image_url: b.featured_image_url,
            author_id: b.author_id,
            status: BlogPostStatus::from(b.status),
            publish_date: b.publish_date,
            is_featured: b.is_featured,
            view_count: b.view_count,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostSummaryResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub category: BlogCategory,
    pub tags: Vec<String>,
    pub featured_image_url: Option<String>,
    pub author_id: String,
    pub publish_date: Option<String>,
    pub is_featured: bool,
    pub view_count: i32,
}

impl From<BlogPost> for BlogPostSummaryResponse {
    fn from(b: BlogPost) -> Self {
        let tags: Vec<String> = b
            .tags
            .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        BlogPostSummaryResponse {
            id: b.id,
            title: b.title,
            slug: b.slug,
            summary: b.summary,
            category: BlogCategory::from(b.category),
            tags,
            featured_image_url: b.featured_image_url,
            author_id: b.author_id,
            publish_date: b.publish_date,
            is_featured: b.is_featured,
            view_count: b.view_count,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateBlogPostRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    pub category: Option<BlogCategory>,
    pub tags: Option<Vec<String>>,
    pub featured_image_url: Option<String>,
    pub is_featured: Option<bool>,
    pub publish_immediately: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBlogPostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub category: Option<BlogCategory>,
    pub tags: Option<Vec<String>>,
    pub featured_image_url: Option<String>,
    pub status: Option<BlogPostStatus>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlogPostListResponse {
    pub posts: Vec<BlogPostSummaryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlogPostQuery {
    pub status: Option<String>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub featured_only: Option<bool>,
    pub author_id: Option<String>,
}
