mod common;

use actix_web::test;
use common::{create_test_app, get_admin_token_helper, setup_test_db};

#[actix_rt::test]
async fn test_create_course() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    let req = test::TestRequest::post()
        .uri("/api/courses")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Boxing Fundamentals",
            "description": "Learn the basics of boxing",
            "course_type": "group_class",
            "difficulty_level": "beginner",
            "max_capacity": 20,
            "duration_minutes": 60,
            "price": 25.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert_eq!(json["data"]["name"].as_str().unwrap(), "Boxing Fundamentals");
}

#[actix_rt::test]
async fn test_list_courses() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a course first
    let create_req = test::TestRequest::post()
        .uri("/api/courses")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Test Course",
            "course_type": "group_class",
            "max_capacity": 10,
            "duration_minutes": 45,
            "price": 20.0
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // List courses (public endpoint)
    let req = test::TestRequest::get()
        .uri("/api/courses")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert!(json["data"]["courses"].as_array().unwrap().len() > 0);
}

#[actix_rt::test]
async fn test_get_course_by_id() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a course
    let create_req = test::TestRequest::post()
        .uri("/api/courses")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Advanced Boxing",
            "course_type": "group_class",
            "max_capacity": 15,
            "duration_minutes": 90,
            "price": 35.0
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let course_id = json["data"]["id"].as_str().unwrap();

    // Get course by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/courses/{}", course_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["name"].as_str().unwrap(), "Advanced Boxing");
}

#[actix_rt::test]
async fn test_update_course() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a course
    let create_req = test::TestRequest::post()
        .uri("/api/courses")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Original Name",
            "course_type": "group_class",
            "max_capacity": 10,
            "duration_minutes": 60,
            "price": 30.0
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let course_id = json["data"]["id"].as_str().unwrap();

    // Update course
    let req = test::TestRequest::put()
        .uri(&format!("/api/courses/{}", course_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Updated Name",
            "price": 40.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["name"].as_str().unwrap(), "Updated Name");
    assert_eq!(json["data"]["price"].as_f64().unwrap(), 40.0);
}

#[actix_rt::test]
async fn test_delete_course() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a course
    let create_req = test::TestRequest::post()
        .uri("/api/courses")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Course to Delete",
            "course_type": "workshop",
            "max_capacity": 5,
            "duration_minutes": 120,
            "price": 50.0
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let course_id = json["data"]["id"].as_str().unwrap();

    // Delete course
    let req = test::TestRequest::delete()
        .uri(&format!("/api/courses/{}", course_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify course is deleted
    let get_req = test::TestRequest::get()
        .uri(&format!("/api/courses/{}", course_id))
        .to_request();
    let resp = test::call_service(&app, get_req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_create_course_without_auth() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .uri("/api/courses")
        .set_json(serde_json::json!({
            "name": "Unauthorized Course",
            "course_type": "group_class",
            "max_capacity": 10,
            "duration_minutes": 60,
            "price": 25.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}
