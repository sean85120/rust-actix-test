use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<String> for DayOfWeek {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "monday" => DayOfWeek::Monday,
            "tuesday" => DayOfWeek::Tuesday,
            "wednesday" => DayOfWeek::Wednesday,
            "thursday" => DayOfWeek::Thursday,
            "friday" => DayOfWeek::Friday,
            "saturday" => DayOfWeek::Saturday,
            "sunday" => DayOfWeek::Sunday,
            _ => DayOfWeek::Monday,
        }
    }
}

impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            DayOfWeek::Monday => "monday".to_string(),
            DayOfWeek::Tuesday => "tuesday".to_string(),
            DayOfWeek::Wednesday => "wednesday".to_string(),
            DayOfWeek::Thursday => "thursday".to_string(),
            DayOfWeek::Friday => "friday".to_string(),
            DayOfWeek::Saturday => "saturday".to_string(),
            DayOfWeek::Sunday => "sunday".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    Scheduled,
    Cancelled,
    Completed,
}

impl Default for ScheduleStatus {
    fn default() -> Self {
        ScheduleStatus::Scheduled
    }
}

impl From<String> for ScheduleStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "scheduled" => ScheduleStatus::Scheduled,
            "cancelled" => ScheduleStatus::Cancelled,
            "completed" => ScheduleStatus::Completed,
            _ => ScheduleStatus::Scheduled,
        }
    }
}

impl ToString for ScheduleStatus {
    fn to_string(&self) -> String {
        match self {
            ScheduleStatus::Scheduled => "scheduled".to_string(),
            ScheduleStatus::Cancelled => "cancelled".to_string(),
            ScheduleStatus::Completed => "completed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseSchedule {
    pub id: String,
    pub course_id: String,
    pub instructor_id: Option<String>,
    pub scheduled_date: String,
    pub start_time: String,
    pub end_time: String,
    pub max_capacity: i32,
    pub current_enrollment: i32,
    pub location: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseScheduleResponse {
    pub id: String,
    pub course_id: String,
    pub course_name: Option<String>,
    pub instructor_id: Option<String>,
    pub instructor_name: Option<String>,
    pub scheduled_date: String,
    pub start_time: String,
    pub end_time: String,
    pub max_capacity: i32,
    pub current_enrollment: i32,
    pub available_spots: i32,
    pub location: Option<String>,
    pub status: ScheduleStatus,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CourseSchedule> for CourseScheduleResponse {
    fn from(s: CourseSchedule) -> Self {
        let available = s.max_capacity - s.current_enrollment;
        CourseScheduleResponse {
            id: s.id,
            course_id: s.course_id,
            course_name: None,
            instructor_id: s.instructor_id,
            instructor_name: None,
            scheduled_date: s.scheduled_date,
            start_time: s.start_time,
            end_time: s.end_time,
            max_capacity: s.max_capacity,
            current_enrollment: s.current_enrollment,
            available_spots: available,
            location: s.location,
            status: ScheduleStatus::from(s.status),
            notes: s.notes,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateScheduleRequest {
    pub course_id: String,
    pub instructor_id: Option<String>,
    pub scheduled_date: String,
    pub start_time: String,
    pub end_time: String,
    #[validate(range(min = 1, message = "Max capacity must be at least 1"))]
    pub max_capacity: Option<i32>,
    pub location: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateScheduleRequest {
    pub instructor_id: Option<String>,
    pub scheduled_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub max_capacity: Option<i32>,
    pub location: Option<String>,
    pub status: Option<ScheduleStatus>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleQuery {
    pub course_id: Option<String>,
    pub instructor_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleListResponse {
    pub schedules: Vec<CourseScheduleResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
