use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    CreateMemberRequest, Member, MemberRole, MemberStatus, UpdateMemberRequest,
};

pub async fn create_member(
    pool: &SqlitePool,
    req: &CreateMemberRequest,
    password_hash: &str,
) -> Result<Member, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO members (
            id, email, password_hash, first_name, last_name, phone,
            date_of_birth, emergency_contact_name, emergency_contact_phone,
            status, role, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.email)
    .bind(password_hash)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.phone)
    .bind(&req.date_of_birth)
    .bind(&req.emergency_contact_name)
    .bind(&req.emergency_contact_phone)
    .bind(MemberStatus::Active.to_string())
    .bind(MemberRole::Member.to_string())
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_member_by_id(pool, &id).await
}

pub async fn get_member_by_id(pool: &SqlitePool, id: &str) -> Result<Member, AppError> {
    sqlx::query_as::<_, Member>("SELECT * FROM members WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Member not found".to_string()))
}

pub async fn get_member_by_email(pool: &SqlitePool, email: &str) -> Result<Member, AppError> {
    sqlx::query_as::<_, Member>("SELECT * FROM members WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Member not found".to_string()))
}

pub async fn email_exists(pool: &SqlitePool, email: &str) -> Result<bool, AppError> {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM members WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(result.0 > 0)
}

pub async fn update_member(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateMemberRequest,
) -> Result<Member, AppError> {
    let existing = get_member_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let email = req.email.as_ref().unwrap_or(&existing.email);
    let first_name = req.first_name.as_ref().unwrap_or(&existing.first_name);
    let last_name = req.last_name.as_ref().unwrap_or(&existing.last_name);
    let phone = req.phone.clone().or(existing.phone);
    let date_of_birth = req.date_of_birth.clone().or(existing.date_of_birth);
    let emergency_contact_name = req.emergency_contact_name.clone().or(existing.emergency_contact_name);
    let emergency_contact_phone = req.emergency_contact_phone.clone().or(existing.emergency_contact_phone);
    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status);

    sqlx::query(
        r#"
        UPDATE members SET
            email = ?,
            first_name = ?,
            last_name = ?,
            phone = ?,
            date_of_birth = ?,
            emergency_contact_name = ?,
            emergency_contact_phone = ?,
            status = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(email)
    .bind(first_name)
    .bind(last_name)
    .bind(&phone)
    .bind(&date_of_birth)
    .bind(&emergency_contact_name)
    .bind(&emergency_contact_phone)
    .bind(&status)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_member_by_id(pool, id).await
}

pub async fn delete_member(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM members WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Member not found".to_string()));
    }

    Ok(())
}

pub async fn list_members(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    status_filter: Option<&str>,
    role_filter: Option<&str>,
) -> Result<(Vec<Member>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM members WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM members WHERE 1=1".to_string();

    if let Some(status) = status_filter {
        count_query.push_str(&format!(" AND status = '{}'", status));
        select_query.push_str(&format!(" AND status = '{}'", status));
    }

    if let Some(role) = role_filter {
        count_query.push_str(&format!(" AND role = '{}'", role));
        select_query.push_str(&format!(" AND role = '{}'", role));
    }

    select_query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let members = sqlx::query_as::<_, Member>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((members, total.0))
}

pub async fn assign_membership(
    pool: &SqlitePool,
    member_id: &str,
    plan_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Member, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        UPDATE members SET
            membership_plan_id = ?,
            membership_start_date = ?,
            membership_end_date = ?,
            status = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(plan_id)
    .bind(start_date)
    .bind(end_date)
    .bind(MemberStatus::Active.to_string())
    .bind(&now)
    .bind(member_id)
    .execute(pool)
    .await?;

    get_member_by_id(pool, member_id).await
}

pub async fn update_member_role(
    pool: &SqlitePool,
    member_id: &str,
    role: &MemberRole,
) -> Result<Member, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query("UPDATE members SET role = ?, updated_at = ? WHERE id = ?")
        .bind(role.to_string())
        .bind(&now)
        .bind(member_id)
        .execute(pool)
        .await?;

    get_member_by_id(pool, member_id).await
}

pub async fn get_instructors(pool: &SqlitePool) -> Result<Vec<Member>, AppError> {
    let members = sqlx::query_as::<_, Member>(
        "SELECT * FROM members WHERE role = 'instructor' OR role = 'admin' ORDER BY first_name",
    )
    .fetch_all(pool)
    .await?;

    Ok(members)
}
