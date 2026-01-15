use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Course, CourseStatus, CreateCourseRequest, DifficultyLevel, UpdateCourseRequest,
};

pub async fn create_course(pool: &SqlitePool, req: &CreateCourseRequest) -> Result<Course, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let difficulty = req
        .difficulty_level
        .as_ref()
        .unwrap_or(&DifficultyLevel::AllLevels);

    sqlx::query(
        r#"
        INSERT INTO courses (
            id, name, description, course_type, difficulty_level,
            instructor_id, max_capacity, current_enrollment, duration_minutes,
            price, status, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.course_type.to_string())
    .bind(difficulty.to_string())
    .bind(&req.instructor_id)
    .bind(req.max_capacity)
    .bind(0)
    .bind(req.duration_minutes)
    .bind(req.price)
    .bind(CourseStatus::Active.to_string())
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_course_by_id(pool, &id).await
}

pub async fn get_course_by_id(pool: &SqlitePool, id: &str) -> Result<Course, AppError> {
    sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Course not found".to_string()))
}

pub async fn update_course(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateCourseRequest,
) -> Result<Course, AppError> {
    let existing = get_course_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let name = req.name.as_ref().unwrap_or(&existing.name);
    let description = req.description.clone().or(existing.description);
    let course_type = req
        .course_type
        .as_ref()
        .map(|t| t.to_string())
        .unwrap_or(existing.course_type);
    let difficulty_level = req
        .difficulty_level
        .as_ref()
        .map(|d| d.to_string())
        .unwrap_or(existing.difficulty_level);
    let instructor_id = req.instructor_id.clone().or(existing.instructor_id);
    let max_capacity = req.max_capacity.unwrap_or(existing.max_capacity);
    let duration_minutes = req.duration_minutes.unwrap_or(existing.duration_minutes);
    let price = req.price.unwrap_or(existing.price);
    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status);

    sqlx::query(
        r#"
        UPDATE courses SET
            name = ?,
            description = ?,
            course_type = ?,
            difficulty_level = ?,
            instructor_id = ?,
            max_capacity = ?,
            duration_minutes = ?,
            price = ?,
            status = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(&description)
    .bind(&course_type)
    .bind(&difficulty_level)
    .bind(&instructor_id)
    .bind(max_capacity)
    .bind(duration_minutes)
    .bind(price)
    .bind(&status)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_course_by_id(pool, id).await
}

pub async fn delete_course(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM courses WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Course not found".to_string()));
    }

    Ok(())
}

pub async fn list_courses(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    course_type: Option<&str>,
    difficulty: Option<&str>,
    status: Option<&str>,
) -> Result<(Vec<Course>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM courses WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM courses WHERE 1=1".to_string();

    if let Some(ct) = course_type {
        count_query.push_str(&format!(" AND course_type = '{}'", ct));
        select_query.push_str(&format!(" AND course_type = '{}'", ct));
    }

    if let Some(diff) = difficulty {
        count_query.push_str(&format!(" AND difficulty_level = '{}'", diff));
        select_query.push_str(&format!(" AND difficulty_level = '{}'", diff));
    }

    if let Some(s) = status {
        count_query.push_str(&format!(" AND status = '{}'", s));
        select_query.push_str(&format!(" AND status = '{}'", s));
    }

    select_query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let courses = sqlx::query_as::<_, Course>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((courses, total.0))
}

pub async fn update_enrollment_count(
    pool: &SqlitePool,
    course_id: &str,
    delta: i32,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE courses SET current_enrollment = current_enrollment + ?, updated_at = ? WHERE id = ?",
    )
    .bind(delta)
    .bind(&now)
    .bind(course_id)
    .execute(pool)
    .await?;

    Ok(())
}
