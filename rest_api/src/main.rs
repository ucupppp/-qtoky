// main.rs
use actix_web::{App, HttpServer};

mod api;
mod config;
mod db;
mod errors;
mod middlewares;
mod models;
mod services;
mod utils;

use api::config as api_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("7878".to_string());

    let db_client = db::mongo::init_db().await.expect("Failed to initialize db");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(api_routes)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
