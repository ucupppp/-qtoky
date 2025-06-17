use crate::errors::ApiError;
use crate::models::user::UserResponse;
use crate::services::user_service::{get_user_service, get_users_service};
use actix_web::{
    HttpResponse, Result,
    web::{Data, Path},
};
use mongodb::Database;

pub async fn get_user_handler(
    path: Path<String>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let user = get_user_service(&user_id, &db).await?;

    let user_response: UserResponse = user.into(); // konversi eksplisit dulu

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": user_response
    })))
}

pub async fn get_users_handler(db: Data<Database>) -> Result<HttpResponse, ApiError> {
    let users = get_users_service(&db).await?;

    let users_response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": users_response
    })))
}
