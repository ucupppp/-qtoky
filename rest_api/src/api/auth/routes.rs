use actix_web::web;

use super::handler::{login_handler, register_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login_handler))
            .route("/register", web::post().to(register_handler)),
    );
}
