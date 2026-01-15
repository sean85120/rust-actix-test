use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Confirmed,
    Cancelled,
    Completed,
    NoShow,
    Waitlisted,
}

impl Default for BookingStatus {
    fn default() -> Self {
        BookingStatus::Confirmed
    }
}

impl From<String> for BookingStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "confirmed" => BookingStatus::Confirmed,
            "cancelled" => BookingStatus::Cancelled,
            "completed" => BookingStatus::Completed,
            "no_show" => BookingStatus::NoShow,
            "waitlisted" => BookingStatus::Waitlisted,
            _ => BookingStatus::Confirmed,
        }
    }
}

impl ToString for BookingStatus {
    fn to_string(&self) -> String {
        match self {
            BookingStatus::Confirmed => "confirmed".to_string(),
            BookingStatus::Cancelled => "cancelled".to_string(),
            BookingStatus::Completed => "completed".to_string(),
            BookingStatus::NoShow => "no_show".to_string(),
            BookingStatus::Waitlisted => "waitlisted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: String,
    pub member_id: String,
    pub schedule_id: String,
    pub status: String,
    pub waitlist_position: Option<i32>,
    pub booked_at: String,
    pub cancelled_at: Option<String>,
    pub cancellation_reason: Option<String>,
    pub attended: bool,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingResponse {
    pub id: String,
    pub member_id: String,
    pub member_name: Option<String>,
    pub schedule_id: String,
    pub course_name: Option<String>,
    pub scheduled_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub instructor_name: Option<String>,
    pub status: BookingStatus,
    pub waitlist_position: Option<i32>,
    pub booked_at: String,
    pub cancelled_at: Option<String>,
    pub cancellation_reason: Option<String>,
    pub attended: bool,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Booking> for BookingResponse {
    fn from(b: Booking) -> Self {
        BookingResponse {
            id: b.id,
            member_id: b.member_id,
            member_name: None,
            schedule_id: b.schedule_id,
            course_name: None,
            scheduled_date: None,
            start_time: None,
            end_time: None,
            instructor_name: None,
            status: BookingStatus::from(b.status),
            waitlist_position: b.waitlist_position,
            booked_at: b.booked_at,
            cancelled_at: b.cancelled_at,
            cancellation_reason: b.cancellation_reason,
            attended: b.attended,
            notes: b.notes,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateBookingRequest {
    pub schedule_id: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelBookingRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBookingRequest {
    pub status: Option<BookingStatus>,
    pub attended: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookingQuery {
    pub member_id: Option<String>,
    pub schedule_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingListResponse {
    pub bookings: Vec<BookingResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistResponse {
    pub schedule_id: String,
    pub waitlist: Vec<WaitlistEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistEntry {
    pub booking_id: String,
    pub member_id: String,
    pub member_name: String,
    pub position: i32,
    pub booked_at: String,
}
