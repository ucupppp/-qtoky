use super::handler::{
    delete_user_handler, get_user_handler, get_users_handler, patch_user_handler, post_user_handler,
};
use crate::middlewares::auth_middleware::AuthMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthMiddleware)
            .route("", web::get().to(get_users_handler))
            .route("", web::post().to(post_user_handler))
            .route("{id}", web::get().to(get_user_handler))
            .route("{id}", web::patch().to(patch_user_handler))
            .route("{id}", web::delete().to(delete_user_handler)),
    );
}
