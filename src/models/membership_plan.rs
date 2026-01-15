use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanDuration {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    PerSession,
}

impl Default for PlanDuration {
    fn default() -> Self {
        PlanDuration::Monthly
    }
}

impl From<String> for PlanDuration {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "monthly" => PlanDuration::Monthly,
            "quarterly" => PlanDuration::Quarterly,
            "semi_annual" => PlanDuration::SemiAnnual,
            "annual" => PlanDuration::Annual,
            "per_session" => PlanDuration::PerSession,
            _ => PlanDuration::Monthly,
        }
    }
}

impl ToString for PlanDuration {
    fn to_string(&self) -> String {
        match self {
            PlanDuration::Monthly => "monthly".to_string(),
            PlanDuration::Quarterly => "quarterly".to_string(),
            PlanDuration::SemiAnnual => "semi_annual".to_string(),
            PlanDuration::Annual => "annual".to_string(),
            PlanDuration::PerSession => "per_session".to_string(),
        }
    }
}

impl PlanDuration {
    pub fn days(&self) -> i32 {
        match self {
            PlanDuration::Monthly => 30,
            PlanDuration::Quarterly => 90,
            PlanDuration::SemiAnnual => 180,
            PlanDuration::Annual => 365,
            PlanDuration::PerSession => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MembershipPlan {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub duration: String,
    pub price: f64,
    pub max_bookings_per_week: Option<i32>,
    pub max_bookings_per_month: Option<i32>,
    pub includes_personal_training: bool,
    pub personal_training_sessions: i32,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipPlanResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub duration: PlanDuration,
    pub duration_days: i32,
    pub price: f64,
    pub max_bookings_per_week: Option<i32>,
    pub max_bookings_per_month: Option<i32>,
    pub includes_personal_training: bool,
    pub personal_training_sessions: i32,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MembershipPlan> for MembershipPlanResponse {
    fn from(p: MembershipPlan) -> Self {
        let duration = PlanDuration::from(p.duration.clone());
        MembershipPlanResponse {
            id: p.id,
            name: p.name,
            description: p.description,
            duration_days: duration.days(),
            duration,
            price: p.price,
            max_bookings_per_week: p.max_bookings_per_week,
            max_bookings_per_month: p.max_bookings_per_month,
            includes_personal_training: p.includes_personal_training,
            personal_training_sessions: p.personal_training_sessions,
            is_active: p.is_active,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateMembershipPlanRequest {
    #[validate(length(min = 1, message = "Plan name is required"))]
    pub name: String,
    pub description: Option<String>,
    pub duration: PlanDuration,
    #[validate(range(min = 0.0, message = "Price cannot be negative"))]
    pub price: f64,
    pub max_bookings_per_week: Option<i32>,
    pub max_bookings_per_month: Option<i32>,
    pub includes_personal_training: Option<bool>,
    pub personal_training_sessions: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMembershipPlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub duration: Option<PlanDuration>,
    pub price: Option<f64>,
    pub max_bookings_per_week: Option<i32>,
    pub max_bookings_per_month: Option<i32>,
    pub includes_personal_training: Option<bool>,
    pub personal_training_sessions: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipPlanListResponse {
    pub plans: Vec<MembershipPlanResponse>,
    pub total: i64,
}
