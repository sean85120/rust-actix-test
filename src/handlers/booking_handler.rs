use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_auth, require_instructor_or_admin};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, BookingListResponse, BookingQuery, BookingResponse, BookingStatus,
    CancelBookingRequest, CreateBookingRequest, PaginationQuery, UpdateBookingRequest,
};

pub async fn list_bookings(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    query: web::Query<BookingQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (bookings, total) = db::list_bookings(
        &pool,
        offset,
        per_page,
        query.member_id.as_deref(),
        query.schedule_id.as_deref(),
        query.status.as_deref(),
    )
    .await?;

    let response = BookingListResponse {
        bookings: bookings.into_iter().map(BookingResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn create_booking(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateBookingRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if schedule exists and has capacity
    let schedule = db::get_schedule_by_id(&pool, &body.schedule_id).await?;

    // Check if member already has a booking for this schedule
    if let Some(_existing) =
        db::get_booking_for_member_and_schedule(&pool, &auth_user.id, &body.schedule_id).await?
    {
        return Err(AppError::ConflictError(
            "You already have a booking for this class".to_string(),
        ));
    }

    // Check member's booking limits if they have a membership
    let member = db::get_member_by_id(&pool, &auth_user.id).await?;
    if let Some(plan_id) = &member.membership_plan_id {
        let plan = db::get_membership_plan_by_id(&pool, plan_id).await?;

        // Check weekly limit
        if let Some(max_weekly) = plan.max_bookings_per_week {
            let weekly_count = db::count_member_bookings_this_week(&pool, &auth_user.id).await?;
            if weekly_count >= max_weekly {
                return Err(AppError::ValidationError(
                    "You have reached your weekly booking limit".to_string(),
                ));
            }
        }

        // Check monthly limit
        if let Some(max_monthly) = plan.max_bookings_per_month {
            let monthly_count = db::count_member_bookings_this_month(&pool, &auth_user.id).await?;
            if monthly_count >= max_monthly {
                return Err(AppError::ValidationError(
                    "You have reached your monthly booking limit".to_string(),
                ));
            }
        }
    }

    // Determine if booking should be confirmed or waitlisted
    let (status, waitlist_position) = if schedule.current_enrollment < schedule.max_capacity {
        (BookingStatus::Confirmed, None)
    } else {
        let position = db::get_next_waitlist_position(&pool, &body.schedule_id).await?;
        (BookingStatus::Waitlisted, Some(position))
    };

    let booking = db::create_booking(&pool, &auth_user.id, &body, status.clone(), waitlist_position).await?;

    // Update enrollment count if confirmed
    if status == BookingStatus::Confirmed {
        db::update_schedule_enrollment(&pool, &body.schedule_id, 1).await?;
    }

    let message = if status == BookingStatus::Waitlisted {
        format!(
            "Added to waitlist at position {}",
            waitlist_position.unwrap()
        )
    } else {
        "Booking confirmed".to_string()
    };

    Ok(HttpResponse::Created().json(ApiResponse::success_with_message(
        BookingResponse::from(booking),
        &message,
    )))
}

pub async fn get_booking(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    let booking_id = path.into_inner();

    let booking = db::get_booking_by_id(&pool, &booking_id).await?;

    // Users can only view their own bookings unless admin/instructor
    if booking.member_id != auth_user.id
        && auth_user.role != crate::models::MemberRole::Admin
        && auth_user.role != crate::models::MemberRole::Instructor
    {
        return Err(AppError::AuthorizationError(
            "Not authorized to view this booking".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(BookingResponse::from(booking))))
}

pub async fn update_booking(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateBookingRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    let booking_id = path.into_inner();
    let booking = db::update_booking(&pool, &booking_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(BookingResponse::from(booking))))
}

pub async fn cancel_booking(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: Option<web::Json<CancelBookingRequest>>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    let booking_id = path.into_inner();

    let booking = db::get_booking_by_id(&pool, &booking_id).await?;

    // Users can only cancel their own bookings unless admin
    if booking.member_id != auth_user.id && auth_user.role != crate::models::MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Not authorized to cancel this booking".to_string(),
        ));
    }

    let reason = body.and_then(|b| b.reason.clone());
    let was_confirmed = booking.status == "confirmed";
    let schedule_id = booking.schedule_id.clone();

    let cancelled_booking = db::cancel_booking(&pool, &booking_id, reason).await?;

    // Update enrollment count and promote from waitlist if needed
    if was_confirmed {
        db::update_schedule_enrollment(&pool, &schedule_id, -1).await?;

        // Try to promote someone from the waitlist
        let waitlist = db::get_waitlist_for_schedule(&pool, &schedule_id).await?;
        if let Some(next_in_line) = waitlist.first() {
            db::promote_from_waitlist(&pool, &next_in_line.id).await?;
            db::update_schedule_enrollment(&pool, &schedule_id, 1).await?;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        BookingResponse::from(cancelled_booking),
        "Booking cancelled successfully",
    )))
}

pub async fn get_my_bookings(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;

    let page = query.get_page();
    let per_page = query.get_per_page();
    let offset = query.get_offset();

    let (bookings, total) = db::get_member_bookings(&pool, &auth_user.id, offset, per_page).await?;

    let response = BookingListResponse {
        bookings: bookings.into_iter().map(BookingResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
