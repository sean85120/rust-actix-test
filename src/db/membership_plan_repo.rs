use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    CreateMembershipPlanRequest, MembershipPlan, UpdateMembershipPlanRequest,
};

pub async fn create_membership_plan(
    pool: &SqlitePool,
    req: &CreateMembershipPlanRequest,
) -> Result<MembershipPlan, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO membership_plans (
            id, name, description, duration, price,
            max_bookings_per_week, max_bookings_per_month,
            includes_personal_training, personal_training_sessions,
            is_active, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.duration.to_string())
    .bind(req.price)
    .bind(&req.max_bookings_per_week)
    .bind(&req.max_bookings_per_month)
    .bind(req.includes_personal_training.unwrap_or(false))
    .bind(req.personal_training_sessions.unwrap_or(0))
    .bind(true)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_membership_plan_by_id(pool, &id).await
}

pub async fn get_membership_plan_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<MembershipPlan, AppError> {
    sqlx::query_as::<_, MembershipPlan>("SELECT * FROM membership_plans WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Membership plan not found".to_string()))
}

pub async fn update_membership_plan(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateMembershipPlanRequest,
) -> Result<MembershipPlan, AppError> {
    let existing = get_membership_plan_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let name = req.name.as_ref().unwrap_or(&existing.name);
    let description = req.description.clone().or(existing.description);
    let duration = req
        .duration
        .as_ref()
        .map(|d| d.to_string())
        .unwrap_or(existing.duration);
    let price = req.price.unwrap_or(existing.price);
    let max_bookings_per_week = req.max_bookings_per_week.or(existing.max_bookings_per_week);
    let max_bookings_per_month = req.max_bookings_per_month.or(existing.max_bookings_per_month);
    let includes_personal_training = req
        .includes_personal_training
        .unwrap_or(existing.includes_personal_training);
    let personal_training_sessions = req
        .personal_training_sessions
        .unwrap_or(existing.personal_training_sessions);
    let is_active = req.is_active.unwrap_or(existing.is_active);

    sqlx::query(
        r#"
        UPDATE membership_plans SET
            name = ?,
            description = ?,
            duration = ?,
            price = ?,
            max_bookings_per_week = ?,
            max_bookings_per_month = ?,
            includes_personal_training = ?,
            personal_training_sessions = ?,
            is_active = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(&description)
    .bind(&duration)
    .bind(price)
    .bind(&max_bookings_per_week)
    .bind(&max_bookings_per_month)
    .bind(includes_personal_training)
    .bind(personal_training_sessions)
    .bind(is_active)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_membership_plan_by_id(pool, id).await
}

pub async fn delete_membership_plan(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM membership_plans WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Membership plan not found".to_string()));
    }

    Ok(())
}

pub async fn list_membership_plans(
    pool: &SqlitePool,
    include_inactive: bool,
) -> Result<Vec<MembershipPlan>, AppError> {
    let query = if include_inactive {
        "SELECT * FROM membership_plans ORDER BY price ASC"
    } else {
        "SELECT * FROM membership_plans WHERE is_active = 1 ORDER BY price ASC"
    };

    let plans = sqlx::query_as::<_, MembershipPlan>(query)
        .fetch_all(pool)
        .await?;

    Ok(plans)
}
