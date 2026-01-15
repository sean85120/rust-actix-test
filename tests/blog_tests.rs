mod common;

use actix_web::test;
use common::{create_test_app, get_admin_token_helper, register_and_login, setup_test_db, TestMember};

#[actix_rt::test]
async fn test_create_blog_post() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Boxing Training Tips",
            "content": "Here are some tips for improving your boxing technique...",
            "category": "training",
            "tags": ["boxing", "tips", "training"],
            "summary": "Essential boxing tips for beginners"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert_eq!(json["data"]["title"].as_str().unwrap(), "Boxing Training Tips");
    assert_eq!(json["data"]["status"].as_str().unwrap(), "draft");
    assert!(json["data"]["slug"].as_str().unwrap().contains("boxing-training-tips"));
}

#[actix_rt::test]
async fn test_create_blog_post_publish_immediately() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Breaking News",
            "content": "Important announcement...",
            "category": "news",
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
async fn test_list_admin_blog_posts() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Test Blog Post",
            "content": "Test content"
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // List blog posts
    let req = test::TestRequest::get()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["data"]["posts"].as_array().unwrap().len() > 0);
}

#[actix_rt::test]
async fn test_publish_blog_post() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a draft blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Draft Post",
            "content": "This will be published."
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let post_id = json["data"]["id"].as_str().unwrap();

    // Publish the post
    let req = test::TestRequest::post()
        .uri(&format!("/api/admin/blog/{}/publish", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["status"].as_str().unwrap(), "published");
}

#[actix_rt::test]
async fn test_archive_blog_post() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create and publish a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
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
    let post_id = json["data"]["id"].as_str().unwrap();

    // Archive the post
    let req = test::TestRequest::post()
        .uri(&format!("/api/admin/blog/{}/archive", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["status"].as_str().unwrap(), "archived");
}

#[actix_rt::test]
async fn test_update_blog_post() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Original Title",
            "content": "Original content"
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let post_id = json["data"]["id"].as_str().unwrap();

    // Update the post
    let req = test::TestRequest::put()
        .uri(&format!("/api/admin/blog/{}", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Updated Title",
            "category": "tips",
            "is_featured": true
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["title"].as_str().unwrap(), "Updated Title");
    assert_eq!(json["data"]["category"].as_str().unwrap(), "tips");
    assert!(json["data"]["is_featured"].as_bool().unwrap());
}

#[actix_rt::test]
async fn test_delete_blog_post() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "To Delete",
            "content": "This will be deleted."
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let post_id = json["data"]["id"].as_str().unwrap();

    // Delete the post
    let req = test::TestRequest::delete()
        .uri(&format!("/api/admin/blog/{}", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// Public blog endpoint tests

#[actix_rt::test]
async fn test_list_public_blog_posts() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create and publish a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Public Post",
            "content": "This is visible to everyone.",
            "publish_immediately": true
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // List public blog posts (no auth required)
    let req = test::TestRequest::get()
        .uri("/api/blog")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert!(json["data"]["posts"].as_array().unwrap().len() > 0);
}

#[actix_rt::test]
async fn test_get_public_blog_post_by_slug() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create and publish a blog post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "My Amazing Blog Post",
            "content": "Great content here.",
            "publish_immediately": true
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let slug = json["data"]["slug"].as_str().unwrap();

    // Get by slug (no auth required)
    let req = test::TestRequest::get()
        .uri(&format!("/api/blog/{}", slug))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["title"].as_str().unwrap(), "My Amazing Blog Post");
}

#[actix_rt::test]
async fn test_get_draft_post_publicly_fails() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a draft blog post (not published)
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Draft Only Post",
            "content": "This should not be visible publicly."
        }))
        .to_request();
    let resp = test::call_service(&app, create_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let slug = json["data"]["slug"].as_str().unwrap();

    // Try to get draft post publicly
    let req = test::TestRequest::get()
        .uri(&format!("/api/blog/{}", slug))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_featured_posts() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a featured published post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Featured Post",
            "content": "This is a featured post.",
            "is_featured": true,
            "publish_immediately": true
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // Get featured posts
    let req = test::TestRequest::get()
        .uri("/api/blog/featured")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
}

#[actix_rt::test]
async fn test_get_recent_posts() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    let token = get_admin_token_helper(&app, &pool).await;

    // Create a published post
    let create_req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Recent Post",
            "content": "This is a recent post.",
            "publish_immediately": true
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // Get recent posts
    let req = test::TestRequest::get()
        .uri("/api/blog/recent")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
}

#[actix_rt::test]
async fn test_blog_post_without_auth() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .set_json(serde_json::json!({
            "title": "Unauthorized",
            "content": "Should fail"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_blog_post_as_non_admin() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("regular");
    let token = register_and_login(&app, &member).await.unwrap();

    let req = test::TestRequest::post()
        .uri("/api/admin/blog")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "title": "Non-admin",
            "content": "Should fail"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}
