use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_admin, require_auth};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, BlogPostListResponse, BlogPostQuery, BlogPostResponse, BlogPostSummaryResponse,
    CreateBlogPostRequest, MessageResponse, PaginationQuery, UpdateBlogPostRequest,
};

// Admin endpoints

pub async fn create_blog_post(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateBlogPostRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let post = db::create_blog_post(&pool, &body, &auth_user.id).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(BlogPostResponse::from(post))))
}

pub async fn list_blog_posts_admin(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    pagination: web::Query<PaginationQuery>,
    query: web::Query<BlogPostQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let (posts, total) = db::list_blog_posts(
        &pool,
        pagination.get_offset(),
        pagination.get_per_page(),
        query.status.as_deref(),
        query.category.as_deref(),
        query.tag.as_deref(),
        query.featured_only.unwrap_or(false),
        query.author_id.as_deref(),
    )
    .await?;

    let response = BlogPostListResponse {
        posts: posts
            .into_iter()
            .map(BlogPostSummaryResponse::from)
            .collect(),
        total,
        page: pagination.get_page(),
        per_page: pagination.get_per_page(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_blog_post_admin(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let post_id = path.into_inner();
    let post = db::get_blog_post_by_id(&pool, &post_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(BlogPostResponse::from(post))))
}

pub async fn update_blog_post(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateBlogPostRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let post_id = path.into_inner();
    let post = db::update_blog_post(&pool, &post_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(BlogPostResponse::from(post))))
}

pub async fn delete_blog_post(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let post_id = path.into_inner();
    db::delete_blog_post(&pool, &post_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Blog post deleted successfully")))
}

pub async fn publish_blog_post(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let post_id = path.into_inner();
    let post = db::publish_blog_post(&pool, &post_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        BlogPostResponse::from(post),
        "Blog post published successfully",
    )))
}

pub async fn archive_blog_post(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let post_id = path.into_inner();
    let post = db::archive_blog_post(&pool, &post_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        BlogPostResponse::from(post),
        "Blog post archived successfully",
    )))
}

// Public endpoints

#[derive(Debug, serde::Deserialize)]
pub struct PublicBlogQuery {
    pub category: Option<String>,
    pub tag: Option<String>,
}

pub async fn list_public_blog_posts(
    pool: web::Data<DbPool>,
    pagination: web::Query<PaginationQuery>,
    query: web::Query<PublicBlogQuery>,
) -> Result<HttpResponse, AppError> {
    let (posts, total) = db::list_published_blog_posts(
        &pool,
        pagination.get_offset(),
        pagination.get_per_page(),
        query.category.as_deref(),
        query.tag.as_deref(),
    )
    .await?;

    let response = BlogPostListResponse {
        posts: posts
            .into_iter()
            .map(BlogPostSummaryResponse::from)
            .collect(),
        total,
        page: pagination.get_page(),
        per_page: pagination.get_per_page(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_public_blog_post(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let slug = path.into_inner();
    let post = db::get_blog_post_by_slug(&pool, &slug).await?;

    // Only allow viewing published posts publicly
    if post.status != "published" {
        return Err(AppError::NotFound("Blog post not found".to_string()));
    }

    // Increment view count
    let _ = db::increment_view_count(&pool, &post.id).await;

    Ok(HttpResponse::Ok().json(ApiResponse::success(BlogPostResponse::from(post))))
}

pub async fn get_featured_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let posts = db::get_featured_posts(&pool, 5).await?;

    let response: Vec<BlogPostSummaryResponse> = posts
        .into_iter()
        .map(BlogPostSummaryResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_recent_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let posts = db::get_recent_posts(&pool, 10).await?;

    let response: Vec<BlogPostSummaryResponse> = posts
        .into_iter()
        .map(BlogPostSummaryResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
