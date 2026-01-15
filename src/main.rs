use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config = Config::from_env();

    info!("Starting Boxing Gym Backend Server");
    info!("Connecting to database: {}", config.database_url);

    // Create database pool
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Initialize database tables
    db::init_database(&pool)
        .await
        .expect("Failed to initialize database");

    info!("Database initialized successfully");

    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    info!("Server starting at http://{}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(handlers::configure_routes)
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
