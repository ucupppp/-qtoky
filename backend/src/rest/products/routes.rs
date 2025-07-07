use super::handler::{
    delete_product_handler, get_product_handler, get_products_handler, patch_product_handler,
    post_product_handler,
};
use crate::middlewares::auth_middleware::AuthMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .wrap(AuthMiddleware)
            .route("", web::get().to(get_products_handler))
            .route("", web::post().to(post_product_handler))
            .route("{id}", web::get().to(get_product_handler))
            .route("{id}", web::patch().to(patch_product_handler))
            .route("{id}", web::delete().to(delete_product_handler)),
    );
}
