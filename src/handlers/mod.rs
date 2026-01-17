pub mod announcement_handler;
pub mod auth_handler;
pub mod blog_handler;
pub mod booking_handler;
pub mod course_handler;
pub mod member_handler;
pub mod membership_plan_handler;
pub mod schedule_handler;

use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::models::HealthResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            // Auth routes
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth_handler::register))
                    .route("/login", web::post().to(auth_handler::login))
                    .route("/me", web::get().to(auth_handler::get_current_user)),
            )
            // Member routes
            .service(
                web::scope("/members")
                    .route("", web::get().to(member_handler::list_members))
                    .route("/{id}", web::get().to(member_handler::get_member))
                    .route("/{id}", web::put().to(member_handler::update_member))
                    .route("/{id}", web::delete().to(member_handler::delete_member))
                    .route("/{id}/membership", web::post().to(member_handler::assign_membership))
                    .route("/{id}/role", web::put().to(member_handler::update_role))
                    .route("/{id}/bookings", web::get().to(member_handler::get_member_bookings)),
            )
            // Membership plans routes
            .service(
                web::scope("/membership-plans")
                    .route("", web::get().to(membership_plan_handler::list_plans))
                    .route("", web::post().to(membership_plan_handler::create_plan))
                    .route("/{id}", web::get().to(membership_plan_handler::get_plan))
                    .route("/{id}", web::put().to(membership_plan_handler::update_plan))
                    .route("/{id}", web::delete().to(membership_plan_handler::delete_plan)),
            )
            // Course routes
            .service(
                web::scope("/courses")
                    .route("", web::get().to(course_handler::list_courses))
                    .route("", web::post().to(course_handler::create_course))
                    .route("/{id}", web::get().to(course_handler::get_course))
                    .route("/{id}", web::put().to(course_handler::update_course))
                    .route("/{id}", web::delete().to(course_handler::delete_course))
                    .route("/{id}/schedules", web::get().to(course_handler::get_course_schedules)),
            )
            // Schedule routes
            .service(
                web::scope("/schedules")
                    .route("", web::get().to(schedule_handler::list_schedules))
                    .route("", web::post().to(schedule_handler::create_schedule))
                    .route("/{id}", web::get().to(schedule_handler::get_schedule))
                    .route("/{id}", web::put().to(schedule_handler::update_schedule))
                    .route("/{id}", web::delete().to(schedule_handler::delete_schedule))
                    .route("/{id}/cancel", web::post().to(schedule_handler::cancel_schedule))
                    .route("/{id}/bookings", web::get().to(schedule_handler::get_schedule_bookings)),
            )
            // Booking routes
            .service(
                web::scope("/bookings")
                    .route("", web::get().to(booking_handler::list_bookings))
                    .route("", web::post().to(booking_handler::create_booking))
                    .route("/{id}", web::get().to(booking_handler::get_booking))
                    .route("/{id}", web::put().to(booking_handler::update_booking))
                    .route("/{id}/cancel", web::post().to(booking_handler::cancel_booking))
                    .route("/my", web::get().to(booking_handler::get_my_bookings)),
            )
            // Admin announcement routes
            .service(
                web::scope("/admin/announcements")
                    .route("", web::get().to(announcement_handler::list_announcements))
                    .route("", web::post().to(announcement_handler::create_announcement))
                    .route("/{id}", web::get().to(announcement_handler::get_announcement))
                    .route("/{id}", web::put().to(announcement_handler::update_announcement))
                    .route("/{id}", web::delete().to(announcement_handler::delete_announcement))
                    .route("/{id}/publish", web::post().to(announcement_handler::publish_announcement))
                    .route("/{id}/archive", web::post().to(announcement_handler::archive_announcement)),
            )
            // Public announcement route (for authenticated users)
            .route("/announcements", web::get().to(announcement_handler::get_active_announcements))
            // Admin blog routes
            .service(
                web::scope("/admin/blog")
                    .route("", web::get().to(blog_handler::list_blog_posts_admin))
                    .route("", web::post().to(blog_handler::create_blog_post))
                    .route("/{id}", web::get().to(blog_handler::get_blog_post_admin))
                    .route("/{id}", web::put().to(blog_handler::update_blog_post))
                    .route("/{id}", web::delete().to(blog_handler::delete_blog_post))
                    .route("/{id}/publish", web::post().to(blog_handler::publish_blog_post))
                    .route("/{id}/archive", web::post().to(blog_handler::archive_blog_post)),
            )
            // Public blog routes
            .service(
                web::scope("/blog")
                    .route("", web::get().to(blog_handler::list_public_blog_posts))
                    .route("/featured", web::get().to(blog_handler::get_featured_posts))
                    .route("/recent", web::get().to(blog_handler::get_recent_posts))
                    .route("/{slug}", web::get().to(blog_handler::get_public_blog_post)),
            ),
    );
}
