// src/api/users/handler.rs

use crate::errors::error::ApiError;
use crate::services::user_service::get_users_service;
use actix_web::{HttpResponse, Result, web};
use mongodb::Database;

pub async fn get_user_handler(path: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    if user_id == 0 {
        return Err(ApiError::BadRequest("User ID tidak valid".to_string()));
    }

    if user_id == 404 {
        return Err(ApiError::NotFound);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "id": user_id,
            "name": "John Doe"
        }
    })))
}

pub async fn get_users_handler(db: web::Data<Database>) -> Result<HttpResponse, ApiError> {
    match get_users_service(&db).await {
        Ok(users) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": users
        }))),
        Err(_) => Err(ApiError::BadRequest("Gagal mengambil data user".into())),
    }
}
