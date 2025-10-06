mod controllers;
mod database;
mod dto;
mod mappers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./data.db?mode=rwc".to_string());
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8081".to_string());

    log::info!("Initializing application...");

    let db = database::establish_connection(&database_url)
        .await
        .expect("Failed to connect to database");

    database::run_migrations(&db)
        .await
        .expect("Failed to run migrations");

    let db_data = web::Data::new(db);

    log::info!("Starting server at http://{}:{}", server_host, server_port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(db_data.clone())
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
