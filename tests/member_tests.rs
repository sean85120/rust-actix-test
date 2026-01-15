mod common;

use actix_web::test;
use common::{create_test_app, make_admin, register_and_login, setup_test_db, TestMember};

#[actix_rt::test]
async fn test_list_members_as_admin() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;

    // Register and get admin token
    let admin = TestMember::admin();
    let token = register_and_login(&app, &admin).await.unwrap();

    // Make user admin
    let admin_id_req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, admin_id_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let admin_id = json["data"]["id"].as_str().unwrap();
    make_admin(&pool, admin_id).await;

    // Re-login to get updated token
    let token = common::login_member(&app, &admin.email, &admin.password)
        .await
        .unwrap();

    // List members
    let req = test::TestRequest::get()
        .uri("/api/members")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert!(json["data"]["members"].as_array().is_some());
}

#[actix_rt::test]
async fn test_list_members_as_regular_user() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let member = TestMember::new("member1");
    let token = register_and_login(&app, &member).await.unwrap();

    // Regular users should not be able to list all members
    let req = test::TestRequest::get()
        .uri("/api/members")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_member_by_id() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;

    let admin = TestMember::admin();
    let token = register_and_login(&app, &admin).await.unwrap();

    // Get admin ID
    let me_req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, me_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let member_id = json["data"]["id"].as_str().unwrap().to_string();
    make_admin(&pool, &member_id).await;

    // Re-login
    let token = common::login_member(&app, &admin.email, &admin.password)
        .await
        .unwrap();

    // Get member by ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/members/{}", member_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["success"].as_bool().unwrap_or(false));
    assert_eq!(json["data"]["id"].as_str().unwrap(), member_id);
}

#[actix_rt::test]
async fn test_update_member() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;

    let admin = TestMember::admin();
    let token = register_and_login(&app, &admin).await.unwrap();

    // Get admin ID
    let me_req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, me_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let member_id = json["data"]["id"].as_str().unwrap().to_string();
    make_admin(&pool, &member_id).await;

    // Re-login
    let token = common::login_member(&app, &admin.email, &admin.password)
        .await
        .unwrap();

    // Update member
    let req = test::TestRequest::put()
        .uri(&format!("/api/members/{}", member_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "first_name": "UpdatedFirst",
            "last_name": "UpdatedLast"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["first_name"].as_str().unwrap(), "UpdatedFirst");
    assert_eq!(json["data"]["last_name"].as_str().unwrap(), "UpdatedLast");
}

#[actix_rt::test]
async fn test_get_nonexistent_member() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;

    let admin = TestMember::admin();
    let token = register_and_login(&app, &admin).await.unwrap();

    // Get admin ID and make admin
    let me_req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, me_req).await;
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let admin_id = json["data"]["id"].as_str().unwrap();
    make_admin(&pool, admin_id).await;

    let token = common::login_member(&app, &admin.email, &admin.password)
        .await
        .unwrap();

    // Try to get nonexistent member
    let req = test::TestRequest::get()
        .uri("/api/members/nonexistent-id")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(!resp.status().is_success());
}
