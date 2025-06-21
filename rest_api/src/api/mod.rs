use actix_web::web;
mod auth;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::routes::config)
            .configure(auth::routes::config),
    );
}
