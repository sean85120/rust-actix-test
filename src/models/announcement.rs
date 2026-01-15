use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for AnnouncementPriority {
    fn default() -> Self {
        AnnouncementPriority::Normal
    }
}

impl From<String> for AnnouncementPriority {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "low" => AnnouncementPriority::Low,
            "high" => AnnouncementPriority::High,
            "urgent" => AnnouncementPriority::Urgent,
            _ => AnnouncementPriority::Normal,
        }
    }
}

impl ToString for AnnouncementPriority {
    fn to_string(&self) -> String {
        match self {
            AnnouncementPriority::Low => "low".to_string(),
            AnnouncementPriority::Normal => "normal".to_string(),
            AnnouncementPriority::High => "high".to_string(),
            AnnouncementPriority::Urgent => "urgent".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementStatus {
    Draft,
    Published,
    Archived,
}

impl Default for AnnouncementStatus {
    fn default() -> Self {
        AnnouncementStatus::Draft
    }
}

impl From<String> for AnnouncementStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "published" => AnnouncementStatus::Published,
            "archived" => AnnouncementStatus::Archived,
            _ => AnnouncementStatus::Draft,
        }
    }
}

impl ToString for AnnouncementStatus {
    fn to_string(&self) -> String {
        match self {
            AnnouncementStatus::Draft => "draft".to_string(),
            AnnouncementStatus::Published => "published".to_string(),
            AnnouncementStatus::Archived => "archived".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TargetAudience {
    All,
    Members,
    Instructors,
    Staff,
}

impl Default for TargetAudience {
    fn default() -> Self {
        TargetAudience::All
    }
}

impl From<String> for TargetAudience {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "members" => TargetAudience::Members,
            "instructors" => TargetAudience::Instructors,
            "staff" => TargetAudience::Staff,
            _ => TargetAudience::All,
        }
    }
}

impl ToString for TargetAudience {
    fn to_string(&self) -> String {
        match self {
            TargetAudience::All => "all".to_string(),
            TargetAudience::Members => "members".to_string(),
            TargetAudience::Instructors => "instructors".to_string(),
            TargetAudience::Staff => "staff".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub content: String,
    pub priority: String,
    pub status: String,
    pub target_audience: String,
    pub author_id: String,
    pub publish_date: Option<String>,
    pub expiry_date: Option<String>,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub priority: AnnouncementPriority,
    pub status: AnnouncementStatus,
    pub target_audience: TargetAudience,
    pub author_id: String,
    pub publish_date: Option<String>,
    pub expiry_date: Option<String>,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Announcement> for AnnouncementResponse {
    fn from(a: Announcement) -> Self {
        AnnouncementResponse {
            id: a.id,
            title: a.title,
            content: a.content,
            priority: AnnouncementPriority::from(a.priority),
            status: AnnouncementStatus::from(a.status),
            target_audience: TargetAudience::from(a.target_audience),
            author_id: a.author_id,
            publish_date: a.publish_date,
            expiry_date: a.expiry_date,
            is_pinned: a.is_pinned,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateAnnouncementRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    pub priority: Option<AnnouncementPriority>,
    pub target_audience: Option<TargetAudience>,
    pub publish_date: Option<String>,
    pub expiry_date: Option<String>,
    pub is_pinned: Option<bool>,
    pub publish_immediately: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAnnouncementRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub priority: Option<AnnouncementPriority>,
    pub status: Option<AnnouncementStatus>,
    pub target_audience: Option<TargetAudience>,
    pub publish_date: Option<String>,
    pub expiry_date: Option<String>,
    pub is_pinned: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnnouncementListResponse {
    pub announcements: Vec<AnnouncementResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnnouncementQuery {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub target_audience: Option<String>,
    pub pinned_only: Option<bool>,
}
