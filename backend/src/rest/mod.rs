use actix_web::web;
mod auth;
mod products;
mod users;
mod sales;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::routes::config)
            .configure(auth::routes::config)
            .configure(products::routes::config)
            .configure(sales::routes::config),
    );
}


