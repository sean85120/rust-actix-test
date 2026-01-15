use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CourseType {
    GroupClass,
    PersonalTraining,
    OpenGym,
    Workshop,
    Competition,
}

impl Default for CourseType {
    fn default() -> Self {
        CourseType::GroupClass
    }
}

impl From<String> for CourseType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "group_class" => CourseType::GroupClass,
            "personal_training" => CourseType::PersonalTraining,
            "open_gym" => CourseType::OpenGym,
            "workshop" => CourseType::Workshop,
            "competition" => CourseType::Competition,
            _ => CourseType::GroupClass,
        }
    }
}

impl ToString for CourseType {
    fn to_string(&self) -> String {
        match self {
            CourseType::GroupClass => "group_class".to_string(),
            CourseType::PersonalTraining => "personal_training".to_string(),
            CourseType::OpenGym => "open_gym".to_string(),
            CourseType::Workshop => "workshop".to_string(),
            CourseType::Competition => "competition".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    AllLevels,
}

impl Default for DifficultyLevel {
    fn default() -> Self {
        DifficultyLevel::AllLevels
    }
}

impl From<String> for DifficultyLevel {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "beginner" => DifficultyLevel::Beginner,
            "intermediate" => DifficultyLevel::Intermediate,
            "advanced" => DifficultyLevel::Advanced,
            _ => DifficultyLevel::AllLevels,
        }
    }
}

impl ToString for DifficultyLevel {
    fn to_string(&self) -> String {
        match self {
            DifficultyLevel::Beginner => "beginner".to_string(),
            DifficultyLevel::Intermediate => "intermediate".to_string(),
            DifficultyLevel::Advanced => "advanced".to_string(),
            DifficultyLevel::AllLevels => "all_levels".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CourseStatus {
    Active,
    Cancelled,
    Completed,
    Full,
}

impl Default for CourseStatus {
    fn default() -> Self {
        CourseStatus::Active
    }
}

impl From<String> for CourseStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => CourseStatus::Active,
            "cancelled" => CourseStatus::Cancelled,
            "completed" => CourseStatus::Completed,
            "full" => CourseStatus::Full,
            _ => CourseStatus::Active,
        }
    }
}

impl ToString for CourseStatus {
    fn to_string(&self) -> String {
        match self {
            CourseStatus::Active => "active".to_string(),
            CourseStatus::Cancelled => "cancelled".to_string(),
            CourseStatus::Completed => "completed".to_string(),
            CourseStatus::Full => "full".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub course_type: String,
    pub difficulty_level: String,
    pub instructor_id: Option<String>,
    pub max_capacity: i32,
    pub current_enrollment: i32,
    pub duration_minutes: i32,
    pub price: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub course_type: CourseType,
    pub difficulty_level: DifficultyLevel,
    pub instructor_id: Option<String>,
    pub instructor_name: Option<String>,
    pub max_capacity: i32,
    pub current_enrollment: i32,
    pub available_spots: i32,
    pub duration_minutes: i32,
    pub price: f64,
    pub status: CourseStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Course> for CourseResponse {
    fn from(c: Course) -> Self {
        let available = c.max_capacity - c.current_enrollment;
        CourseResponse {
            id: c.id,
            name: c.name,
            description: c.description,
            course_type: CourseType::from(c.course_type),
            difficulty_level: DifficultyLevel::from(c.difficulty_level),
            instructor_id: c.instructor_id,
            instructor_name: None,
            max_capacity: c.max_capacity,
            current_enrollment: c.current_enrollment,
            available_spots: available,
            duration_minutes: c.duration_minutes,
            price: c.price,
            status: CourseStatus::from(c.status),
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[validate(length(min = 1, message = "Course name is required"))]
    pub name: String,
    pub description: Option<String>,
    pub course_type: CourseType,
    pub difficulty_level: Option<DifficultyLevel>,
    pub instructor_id: Option<String>,
    #[validate(range(min = 1, message = "Max capacity must be at least 1"))]
    pub max_capacity: i32,
    #[validate(range(min = 15, message = "Duration must be at least 15 minutes"))]
    pub duration_minutes: i32,
    #[validate(range(min = 0.0, message = "Price cannot be negative"))]
    pub price: f64,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateCourseRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub course_type: Option<CourseType>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub instructor_id: Option<String>,
    #[validate(range(min = 1, message = "Max capacity must be at least 1"))]
    pub max_capacity: Option<i32>,
    #[validate(range(min = 15, message = "Duration must be at least 15 minutes"))]
    pub duration_minutes: Option<i32>,
    #[validate(range(min = 0.0, message = "Price cannot be negative"))]
    pub price: Option<f64>,
    pub status: Option<CourseStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseListResponse {
    pub courses: Vec<CourseResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
