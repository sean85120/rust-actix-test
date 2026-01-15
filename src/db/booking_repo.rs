use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Booking, BookingStatus, CreateBookingRequest, UpdateBookingRequest,
};

pub async fn create_booking(
    pool: &SqlitePool,
    member_id: &str,
    req: &CreateBookingRequest,
    status: BookingStatus,
    waitlist_position: Option<i32>,
) -> Result<Booking, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO bookings (
            id, member_id, schedule_id, status, waitlist_position,
            booked_at, attended, notes, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(member_id)
    .bind(&req.schedule_id)
    .bind(status.to_string())
    .bind(waitlist_position)
    .bind(&now)
    .bind(false)
    .bind(&req.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_booking_by_id(pool, &id).await
}

pub async fn get_booking_by_id(pool: &SqlitePool, id: &str) -> Result<Booking, AppError> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Booking not found".to_string()))
}

pub async fn get_booking_for_member_and_schedule(
    pool: &SqlitePool,
    member_id: &str,
    schedule_id: &str,
) -> Result<Option<Booking>, AppError> {
    let booking = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE member_id = ? AND schedule_id = ? AND status NOT IN ('cancelled')",
    )
    .bind(member_id)
    .bind(schedule_id)
    .fetch_optional(pool)
    .await?;

    Ok(booking)
}

pub async fn update_booking(
    pool: &SqlitePool,
    id: &str,
    req: &UpdateBookingRequest,
) -> Result<Booking, AppError> {
    let existing = get_booking_by_id(pool, id).await?;
    let now = Utc::now().to_rfc3339();

    let status = req
        .status
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(existing.status);
    let attended = req.attended.unwrap_or(existing.attended);
    let notes = req.notes.clone().or(existing.notes);

    sqlx::query(
        r#"
        UPDATE bookings SET
            status = ?,
            attended = ?,
            notes = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&status)
    .bind(attended)
    .bind(&notes)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_booking_by_id(pool, id).await
}

pub async fn cancel_booking(
    pool: &SqlitePool,
    id: &str,
    reason: Option<String>,
) -> Result<Booking, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        UPDATE bookings SET
            status = 'cancelled',
            cancelled_at = ?,
            cancellation_reason = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(&reason)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    get_booking_by_id(pool, id).await
}

pub async fn list_bookings(
    pool: &SqlitePool,
    offset: i64,
    limit: i64,
    member_id: Option<&str>,
    schedule_id: Option<&str>,
    status: Option<&str>,
) -> Result<(Vec<Booking>, i64), AppError> {
    let mut count_query = "SELECT COUNT(*) FROM bookings WHERE 1=1".to_string();
    let mut select_query = "SELECT * FROM bookings WHERE 1=1".to_string();

    if let Some(mid) = member_id {
        count_query.push_str(&format!(" AND member_id = '{}'", mid));
        select_query.push_str(&format!(" AND member_id = '{}'", mid));
    }

    if let Some(sid) = schedule_id {
        count_query.push_str(&format!(" AND schedule_id = '{}'", sid));
        select_query.push_str(&format!(" AND schedule_id = '{}'", sid));
    }

    if let Some(s) = status {
        count_query.push_str(&format!(" AND status = '{}'", s));
        select_query.push_str(&format!(" AND status = '{}'", s));
    }

    select_query.push_str(" ORDER BY booked_at DESC LIMIT ? OFFSET ?");

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;

    let bookings = sqlx::query_as::<_, Booking>(&select_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((bookings, total.0))
}

pub async fn get_member_bookings(
    pool: &SqlitePool,
    member_id: &str,
    offset: i64,
    limit: i64,
) -> Result<(Vec<Booking>, i64), AppError> {
    list_bookings(pool, offset, limit, Some(member_id), None, None).await
}

pub async fn get_schedule_bookings(
    pool: &SqlitePool,
    schedule_id: &str,
) -> Result<Vec<Booking>, AppError> {
    let bookings = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE schedule_id = ? AND status != 'cancelled' ORDER BY booked_at ASC",
    )
    .bind(schedule_id)
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}

pub async fn get_waitlist_for_schedule(
    pool: &SqlitePool,
    schedule_id: &str,
) -> Result<Vec<Booking>, AppError> {
    let bookings = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE schedule_id = ? AND status = 'waitlisted' ORDER BY waitlist_position ASC",
    )
    .bind(schedule_id)
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}

pub async fn get_next_waitlist_position(
    pool: &SqlitePool,
    schedule_id: &str,
) -> Result<i32, AppError> {
    let result: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(waitlist_position) FROM bookings WHERE schedule_id = ? AND status = 'waitlisted'",
    )
    .bind(schedule_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0.unwrap_or(0) + 1)
}

pub async fn promote_from_waitlist(
    pool: &SqlitePool,
    booking_id: &str,
) -> Result<Booking, AppError> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        UPDATE bookings SET
            status = 'confirmed',
            waitlist_position = NULL,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(booking_id)
    .execute(pool)
    .await?;

    get_booking_by_id(pool, booking_id).await
}

pub async fn count_member_bookings_this_week(
    pool: &SqlitePool,
    member_id: &str,
) -> Result<i32, AppError> {
    let result: (i32,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM bookings b
        JOIN course_schedules cs ON b.schedule_id = cs.id
        WHERE b.member_id = ?
        AND b.status IN ('confirmed', 'completed')
        AND cs.scheduled_date >= date('now', 'weekday 0', '-7 days')
        AND cs.scheduled_date < date('now', 'weekday 0')
        "#,
    )
    .bind(member_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

pub async fn count_member_bookings_this_month(
    pool: &SqlitePool,
    member_id: &str,
) -> Result<i32, AppError> {
    let result: (i32,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM bookings b
        JOIN course_schedules cs ON b.schedule_id = cs.id
        WHERE b.member_id = ?
        AND b.status IN ('confirmed', 'completed')
        AND cs.scheduled_date >= date('now', 'start of month')
        AND cs.scheduled_date < date('now', 'start of month', '+1 month')
        "#,
    )
    .bind(member_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}
