mod common;

use actix_web::test;
use common::{create_test_app, get_admin_token_helper, register_and_login, setup_test_db, TestMember};

#[actix_rt::test]
async fn test_create_announcement() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Gym Closure Notice",
            "content": "The gym will be closed on January 20th for maintenance.",
            "priority": "high",
            "target_audience": "all"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert_eq!(json["data"]["title"].as_str().unwrap(), "Gym Closure Notice");
    assert_eq!(json["data"]["status"].as_str().unwrap(), "draft");
}

#[actix_rt::test]
async fn test_create_announcement_publish_immediately() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Urgent Notice",
            "content": "Important message for all members.",
            "priority": "urgent",
            "publish_immediately": true
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["status"].as_str().unwrap(), "published");
}

#[actix_rt::test]
async fn test_list_announcements() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create an announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Test Announcement",
            "content": "Test content"
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // List announcements
    let req = test::TestRequest::get()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["data"]["announcements"].as_array().unwrap().len() > 0);
}

#[actix_rt::test]
async fn test_publish_announcement() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a draft announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Draft Announcement",
            "content": "This will be published."
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let announcement_id = json["data"]["id"].as_str().unwrap();

    // Publish the announcement
    let req = test::TestRequest::post()
        .uri(&format!("/api/admin/announcements/{}/publish", announcement_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["status"].as_str().unwrap(), "published");
}

#[actix_rt::test]
async fn test_archive_announcement() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create and publish an announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "To Archive",
            "content": "This will be archived.",
            "publish_immediately": true
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let announcement_id = json["data"]["id"].as_str().unwrap();

    // Archive the announcement
    let req = test::TestRequest::post()
        .uri(&format!("/api/admin/announcements/{}/archive", announcement_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["status"].as_str().unwrap(), "archived");
}

#[actix_rt::test]
async fn test_update_announcement() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create an announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Original Title",
            "content": "Original content"
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let announcement_id = json["data"]["id"].as_str().unwrap();

    // Update the announcement
    let req = test::TestRequest::put()
        .uri(&format!("/api/admin/announcements/{}", announcement_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Updated Title",
            "priority": "urgent",
            "is_pinned": true
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["title"].as_str().unwrap(), "Updated Title");
    assert_eq!(json["data"]["priority"].as_str().unwrap(), "urgent");
    assert!(json["data"]["is_pinned"].as_bool().unwrap());
}

#[actix_rt::test]
async fn test_delete_announcement() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create an announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "To Delete",
            "content": "This will be deleted."
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let announcement_id = json["data"]["id"].as_str().unwrap();

    // Delete the announcement
    let req = test::TestRequest::delete()
        .uri(&format!("/api/admin/announcements/{}", announcement_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_active_announcements_as_member() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let admin_token = get_admin_token_helper(&app, &pool).await;

    // Create and publish an announcement
    let create_req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(serde_json::json!({
            "title": "Active Announcement",
            "content": "This is visible to members.",
            "publish_immediately": true
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // Register a regular member
    let member = TestMember::new("member1");
    let member_token = register_and_login(&app, &member).await.unwrap();

    // Get active announcements as member
    let req = test::TestRequest::get()
        .uri("/api/announcements")
        .insert_header(("Authorization", format!("Bearer {}", member_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
}

#[actix_rt::test]
async fn test_announcement_without_auth() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .set_json(serde_json::json!({
            "title": "Unauthorized",
            "content": "Should fail"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_announcement_as_non_admin() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("regular");
    let token = register_and_login(&app, &member).await.unwrap();

    let req = test::TestRequest::post()
        .uri("/api/admin/announcements")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Non-admin",
            "content": "Should fail"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}
