pub mod announcement;
pub mod blog;
pub mod booking;
pub mod course;
pub mod member;
pub mod membership_plan;
pub mod schedule;

pub use announcement::*;
pub use blog::*;
pub use booking::*;
pub use course::*;
pub use member::*;
pub use membership_plan::*;
pub use schedule::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationQuery {
    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    pub fn get_offset(&self) -> i64 {
        (self.get_page() - 1) * self.get_per_page()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: Some(message.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            success: true,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}
