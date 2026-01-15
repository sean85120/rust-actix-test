mod common;

use actix_web::test;
use common::{create_test_app, register_member, setup_test_db, TestMember};

#[actix_rt::test]
async fn test_register_success() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("1");
    let result = register_member(&app, &member).await;

    assert!(result["success"].as_bool().unwrap_or(false));
    assert!(result["data"]["member"]["id"].as_str().is_some());
    assert_eq!(result["data"]["member"]["email"].as_str().unwrap(), member.email);
}

#[actix_rt::test]
async fn test_register_duplicate_email() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("2");

    // First registration should succeed
    let result1 = register_member(&app, &member).await;
    assert!(result1["success"].as_bool().unwrap_or(false));

    // Second registration with same email should fail
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": member.email,
            "password": "different123",
            "first_name": "Different",
            "last_name": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_login_success() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("3");
    register_member(&app, &member).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": member.email,
            "password": member.password
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert!(json["data"]["token"].as_str().is_some());
}

#[actix_rt::test]
async fn test_login_wrong_password() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("4");
    register_member(&app, &member).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": member.email,
            "password": "wrongpassword"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_login_nonexistent_user() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "somepassword"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_current_user() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("5");
    register_member(&app, &member).await;

    // Login to get token
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": member.email,
            "password": member.password
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    let body = test::read_body(login_resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = json["data"]["token"].as_str().unwrap();

    // Get current user with token
    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["email"].as_str().unwrap(), member.email);
}

#[actix_rt::test]
async fn test_get_current_user_no_auth() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_register_validation() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    // Test with invalid email
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "invalid-email",
            "password": "password123",
            "first_name": "Test",
            "last_name": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());

    // Test with short password
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "valid@example.com",
            "password": "short",
            "first_name": "Test",
            "last_name": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}
