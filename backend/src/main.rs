// main.rs
use actix_web::{App, HttpServer, middleware::Logger};

mod config;
mod db;
mod errors;
mod middlewares;
mod models;
mod rest;
mod services;
mod utils;

use rest::config as rest_api_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("7878".to_string());

    let db_client = db::mongo::init_db().await.expect("Failed to initialize db");
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(rest_api_routes)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
