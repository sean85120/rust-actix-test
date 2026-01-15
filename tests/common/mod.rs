use actix_web::{test, web, App};
use boxing_gym_backend::config::Config;
use boxing_gym_backend::db::{create_pool, init_database, DbPool};
use boxing_gym_backend::handlers::configure_routes;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use sqlx;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn setup_test_db() -> DbPool {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    // Use file-based temp databases for test isolation
    let db_path = format!("/tmp/test_boxing_gym_{}.db", counter);
    // Remove existing file if present
    let _ = std::fs::remove_file(&db_path);
    let db_url = format!("sqlite:{}?mode=rwc", db_path);
    let pool = create_pool(&db_url).await.expect("Failed to create test pool");
    init_database(&pool).await.expect("Failed to init test database");
    pool
}

pub fn get_test_config() -> Config {
    Config {
        database_url: "sqlite::memory:".to_string(),
        jwt_secret: "test-secret-key-for-testing-purposes".to_string(),
        jwt_expiration_hours: 24,
        server_host: "127.0.0.1".to_string(),
        server_port: 8080,
    }
}

pub async fn create_test_app(
    pool: DbPool,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let config = get_test_config();

    test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(config))
            .configure(configure_routes),
    )
    .await
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestMember {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

impl TestMember {
    pub fn new(suffix: &str) -> Self {
        TestMember {
            email: format!("test{}@example.com", suffix),
            password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: format!("User{}", suffix),
        }
    }

    pub fn admin() -> Self {
        TestMember {
            email: "admin@boxinggym.com".to_string(),
            password: "adminpass123".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
        }
    }
}

pub async fn register_member(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    member: &TestMember,
) -> serde_json::Value {
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": member.email,
            "password": member.password,
            "first_name": member.first_name,
            "last_name": member.last_name
        }))
        .to_request();

    let resp = test::call_service(app, req).await;
    let body = test::read_body(resp).await;
    serde_json::from_slice(&body).unwrap_or(serde_json::json!({}))
}

pub async fn login_member(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    email: &str,
    password: &str,
) -> Option<String> {
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();

    let resp = test::call_service(app, req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).ok()?;
    json["data"]["token"].as_str().map(|s| s.to_string())
}

pub async fn register_and_login(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    member: &TestMember,
) -> Option<String> {
    register_member(app, member).await;
    login_member(app, &member.email, &member.password).await
}

pub async fn make_admin(pool: &DbPool, member_id: &str) {
    sqlx::query("UPDATE members SET role = 'admin' WHERE id = ?")
        .bind(member_id)
        .execute(pool)
        .await
        .expect("Failed to make member admin");
}

pub async fn get_admin_token_helper(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    pool: &DbPool,
) -> String {
    let admin = TestMember::admin();
    let token = register_and_login(app, &admin).await.unwrap();

    let me_req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(app, me_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let admin_id = json["data"]["id"].as_str().unwrap();
    make_admin(pool, admin_id).await;

    login_member(app, &admin.email, &admin.password)
        .await
        .unwrap()
}
