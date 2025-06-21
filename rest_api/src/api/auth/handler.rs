use crate::{
    errors::ApiError,
    models::user::{LoginDTO, UserResponse},
    services::auth_service::login_service,
    utils::jwt::{Claims, create_auth_cookie, create_csrf_cookie, encode_jwt, generate_csrf_token},
};
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json},
};
use mongodb::Database;
use serde_json::json;
use validator::Validate;

pub async fn login_handler(
    payload: Option<Json<LoginDTO>>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    // Validasi login
    let Some(json_payload) = payload else {
        return Err(ApiError::BadRequest("Body tidak boleh kosong".into()));
    };
    let data = json_payload.into_inner();
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
            "message": "Login berhasil",
            "data": user_response,
        })))
}
