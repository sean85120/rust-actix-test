pub mod booking_repo;
pub mod course_repo;
pub mod member_repo;
pub mod membership_plan_repo;
pub mod schedule_repo;

pub use booking_repo::*;
pub use course_repo::*;
pub use member_repo::*;
pub use membership_plan_repo::*;
pub use schedule_repo::*;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;

pub type DbPool = SqlitePool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn init_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Create members table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS members (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            phone TEXT,
            date_of_birth TEXT,
            emergency_contact_name TEXT,
            emergency_contact_phone TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            role TEXT NOT NULL DEFAULT 'member',
            membership_plan_id TEXT,
            membership_start_date TEXT,
            membership_end_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (membership_plan_id) REFERENCES membership_plans(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create membership_plans table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS membership_plans (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            duration TEXT NOT NULL,
            price REAL NOT NULL,
            max_bookings_per_week INTEGER,
            max_bookings_per_month INTEGER,
            includes_personal_training INTEGER NOT NULL DEFAULT 0,
            personal_training_sessions INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create courses table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS courses (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            course_type TEXT NOT NULL,
            difficulty_level TEXT NOT NULL DEFAULT 'all_levels',
            instructor_id TEXT,
            max_capacity INTEGER NOT NULL,
            current_enrollment INTEGER NOT NULL DEFAULT 0,
            duration_minutes INTEGER NOT NULL,
            price REAL NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (instructor_id) REFERENCES members(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create course_schedules table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS course_schedules (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            instructor_id TEXT,
            scheduled_date TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            max_capacity INTEGER NOT NULL,
            current_enrollment INTEGER NOT NULL DEFAULT 0,
            location TEXT,
            status TEXT NOT NULL DEFAULT 'scheduled',
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            FOREIGN KEY (instructor_id) REFERENCES members(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create bookings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bookings (
            id TEXT PRIMARY KEY,
            member_id TEXT NOT NULL,
            schedule_id TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'confirmed',
            waitlist_position INTEGER,
            booked_at TEXT NOT NULL,
            cancelled_at TEXT,
            cancellation_reason TEXT,
            attended INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (member_id) REFERENCES members(id),
            FOREIGN KEY (schedule_id) REFERENCES course_schedules(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_members_email ON members(email)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_members_status ON members(status)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_courses_type ON courses(course_type)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_schedules_date ON course_schedules(scheduled_date)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_schedules_course ON course_schedules(course_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_member ON bookings(member_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookings_schedule ON bookings(schedule_id)")
        .execute(pool)
        .await?;

    log::info!("Database initialized successfully");
    Ok(())
}
