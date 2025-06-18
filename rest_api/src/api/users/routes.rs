use actix_web::web;

use super::handler::{get_user_handler, get_users_handler, patch_user_handler, post_user_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users_handler))
            .route("", web::post().to(post_user_handler))
            .route("{id}", web::get().to(get_user_handler))
            .route("{id}", web::patch().to(patch_user_handler)),
    );
}
