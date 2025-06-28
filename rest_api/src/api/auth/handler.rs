use crate::{
    errors::ApiError,
    models::user::{LoginDTO, RegisterDTO, UserResponse},
    services::auth_service::{login_service, register_service},
    utils::jwt::{Claims, create_auth_cookie, create_csrf_cookie, encode_jwt, generate_csrf_token},
};
use actix_web::{
    Error as ActixError, HttpResponse, Result,
    web::{Data, Json},
};
use mongodb::Database;
use serde_json::json;
use validator::Validate;

pub async fn login_handler(
    payload: Result<Json<LoginDTO>, ActixError>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let data = payload?.into_inner();
    data.validate()?;
    let user = login_service(data, &db).await?;
    let user_response: UserResponse = user.clone().into();
    // Generate JWT & CSRF token
    let claims = Claims {
        sub: user.id.unwrap().to_hex(), // pastikan user.id ada
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let jwt_token = encode_jwt(&claims).map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => {
            ApiError::InternalError("JWT key tidak valid".into())
        }
        _ => ApiError::InternalError(format!("JWT Error: {}", e)),
    })?;

    let csrf_token = generate_csrf_token();

    // Buat cookie untuk auth & csrf
    let auth_cookie = create_auth_cookie(&jwt_token);
    let csrf_cookie = create_csrf_cookie(&csrf_token);
    Ok(HttpResponse::Ok()
        .cookie(auth_cookie)
        .cookie(csrf_cookie)
        .json(json!({
            "status": "success",
            "data": user_response,
            "code":200
        })))
}

pub async fn register_handler(
    payload: Result<Json<RegisterDTO>, ActixError>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let data = payload?.into_inner();
    data.validate()?;
    let user = register_service(data, &db).await?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "data": user,
        "code": 201
    })))
}
