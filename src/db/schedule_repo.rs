use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    CourseSchedule, CreateScheduleRequest, ScheduleStatus, UpdateScheduleRequest,
};

pub async fn create_schedule(
    pool: &SqlitePool,
    req: &CreateScheduleRequest,
    default_capacity: i32,
) -> Result<CourseSchedule, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let max_capacity = req.max_capacity.unwrap_or(default_capacity);

    sqlx::query(
        r#"
        INSERT INTO course_schedules (
            id, course_id, instructor_id, scheduled_date, start_time,
            end_time, max_capacity, current_enrollment, location, status,
            notes, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.course_id)
    .bind(&req.instructor_id)
    .bind(&req.scheduled_date)
    .bind(&req.start_time)
    .bind(&req.end_time)
    .bind(max_capacity)
    .bind(0)
    .bind(&req.location)
    .bind(ScheduleStatus::Scheduled.to_string())
    .bind(&req.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_schedule_by_id(pool, &id).await
}

pub async fn get_schedule_by_id(pool: &SqlitePool, id: &str) -> Result<CourseSchedule, AppError> {
    sqlx::query_as::<_, CourseSchedule>("SELECT * FROM course_schedules WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Schedule not found".to_string()))
}

pub async fn update_schedule(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateScheduleRequest,
) -> Result<CourseSchedule, AppError> {
    let existing = get_schedule_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let instructor_id = req.instructor_id.clone().or(existing.instructor_id);
    let scheduled_date = req.scheduled_date.as_ref().unwrap_or(&existing.scheduled_date);
    let start_time = req.start_time.as_ref().unwrap_or(&existing.start_time);
    let end_time = req.end_time.as_ref().unwrap_or(&existing.end_time);
    let max_capacity = req.max_capacity.unwrap_or(existing.max_capacity);
    let location = req.location.clone().or(existing.location);
    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status);
    let notes = req.notes.clone().or(existing.notes);

    sqlx::query(
        r#"
        UPDATE course_schedules SET
            instructor_id = ?,
            scheduled_date = ?,
            start_time = ?,
            end_time = ?,
            max_capacity = ?,
            location = ?,
            status = ?,
            notes = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&instructor_id)
    .bind(scheduled_date)
    .bind(start_time)
    .bind(end_time)
    .bind(max_capacity)
    .bind(&location)
    .bind(&status)
    .bind(&notes)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_schedule_by_id(pool, id).await
}

pub async fn delete_schedule(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM course_schedules WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Schedule not found".to_string()));
    }

    Ok(())
}

pub async fn list_schedules(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    course_id: Option<&str>,
    instructor_id: Option<&str>,
    date_from: Option<&str>,
    date_to: Option<&str>,
    status: Option<&str>,
) -> Result<(Vec<CourseSchedule>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM course_schedules WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM course_schedules WHERE 1=1".to_string();

    if let Some(cid) = course_id {
        count_query.push_str(&format!(" AND course_id = '{}'", cid));
        select_query.push_str(&format!(" AND course_id = '{}'", cid));
    }

    if let Some(iid) = instructor_id {
        count_query.push_str(&format!(" AND instructor_id = '{}'", iid));
        select_query.push_str(&format!(" AND instructor_id = '{}'", iid));
    }

    if let Some(df) = date_from {
        count_query.push_str(&format!(" AND scheduled_date >= '{}'", df));
        select_query.push_str(&format!(" AND scheduled_date >= '{}'", df));
    }

    if let Some(dt) = date_to {
        count_query.push_str(&format!(" AND scheduled_date <= '{}'", dt));
        select_query.push_str(&format!(" AND scheduled_date <= '{}'", dt));
    }

    if let Some(s) = status {
        count_query.push_str(&format!(" AND status = '{}'", s));
        select_query.push_str(&format!(" AND status = '{}'", s));
    }

    select_query.push_str(" ORDER BY scheduled_date ASC, start_time ASC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let schedules = sqlx::query_as::<_, CourseSchedule>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((schedules, total.0))
}

pub async fn update_schedule_enrollment(
    pool: &SqlitePool,
    schedule_id: &str,
    delta: i32,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE course_schedules SET current_enrollment = current_enrollment + ?, updated_at = ? WHERE id = ?",
    )
    .bind(delta)
    .bind(&now)
    .bind(schedule_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn cancel_schedule(pool: &SqlitePool, id: &str) -> Result<CourseSchedule, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query("UPDATE course_schedules SET status = 'cancelled', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_schedule_by_id(pool, id).await
}
