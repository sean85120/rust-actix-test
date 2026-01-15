use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
}

impl Default for MemberStatus {
    fn default() -> Self {
        MemberStatus::Active
    }
}

impl From<String> for MemberStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => MemberStatus::Active,
            "inactive" => MemberStatus::Inactive,
            "suspended" => MemberStatus::Suspended,
            "expired" => MemberStatus::Expired,
            _ => MemberStatus::Inactive,
        }
    }
}

impl ToString for MemberStatus {
    fn to_string(&self) -> String {
        match self {
            MemberStatus::Active => "active".to_string(),
            MemberStatus::Inactive => "inactive".to_string(),
            MemberStatus::Suspended => "suspended".to_string(),
            MemberStatus::Expired => "expired".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    Member,
    Admin,
    Instructor,
}

impl Default for MemberRole {
    fn default() -> Self {
        MemberRole::Member
    }
}

impl From<String> for MemberRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => MemberRole::Admin,
            "instructor" => MemberRole::Instructor,
            _ => MemberRole::Member,
        }
    }
}

impl ToString for MemberRole {
    fn to_string(&self) -> String {
        match self {
            MemberRole::Member => "member".to_string(),
            MemberRole::Admin => "admin".to_string(),
            MemberRole::Instructor => "instructor".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Member {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub date_of_birth: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub status: String,
    pub role: String,
    pub membership_plan_id: Option<String>,
    pub membership_start_date: Option<String>,
    pub membership_end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub date_of_birth: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub status: MemberStatus,
    pub role: MemberRole,
    pub membership_plan_id: Option<String>,
    pub membership_start_date: Option<String>,
    pub membership_end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Member> for MemberResponse {
    fn from(m: Member) -> Self {
        MemberResponse {
            id: m.id,
            email: m.email,
            first_name: m.first_name,
            last_name: m.last_name,
            phone: m.phone,
            date_of_birth: m.date_of_birth,
            emergency_contact_name: m.emergency_contact_name,
            emergency_contact_phone: m.emergency_contact_phone,
            status: MemberStatus::from(m.status),
            role: MemberRole::from(m.role),
            membership_plan_id: m.membership_plan_id,
            membership_start_date: m.membership_start_date,
            membership_end_date: m.membership_end_date,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,
    pub phone: Option<String>,
    pub date_of_birth: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub status: Option<MemberStatus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssignMembershipRequest {
    pub membership_plan_id: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub member: MemberResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListResponse {
    pub members: Vec<MemberResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
