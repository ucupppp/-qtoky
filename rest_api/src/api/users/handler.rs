use crate::errors::ApiError;
use crate::models::user::{CreateUserDTO, UpdateUserDTO, UserResponse};
use crate::services::user_service::{
    create_user_service, get_user_service, get_users_service, update_user_service,
};
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json, Path},
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
        "data": users_response,
        "code": 200
    })))
}

pub async fn post_user_handler(
    payload: Json<CreateUserDTO>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let new_user = create_user_service(payload.into_inner(), &db).await?;
    let user_response: UserResponse = new_user.into();
    Ok(HttpResponse::Created()
        .append_header(("Location", format!("/users/{}", user_response.id)))
        .json(serde_json::json!({
            "status": "success",
            "data": user_response,
            "code": 201
        })))
}

pub async fn patch_user_handler(
    path: Path<String>,
    payload: Option<Json<UpdateUserDTO>>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    let Some(json_payload) = payload else {
        return Err(ApiError::BadRequest("Body tidak boleh kosong".into()));
    };

    let data = json_payload.into_inner();

    // Validasi semua field kosong atau berisi string kosong
    let no_fields = data
        .username
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
        && data
            .email
            .as_ref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        && data
            .phone_number
            .as_ref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        && data
            .password
            .as_ref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true);

    if no_fields {
        return Err(ApiError::BadRequest(
            "Minimal satu field valid harus diisi".into(),
        ));
    }

    let user = update_user_service(&user_id, data, &db).await?;
    let user_response: UserResponse = user.into();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": user_response,
        "code": 200
    })))
}
