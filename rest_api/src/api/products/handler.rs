use actix_web::{
    Error as ActixError, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path},
};

use crate::errors::ApiError;
use crate::models::product::{ProductDTO, ProductResponse};
use crate::services::product_service::{create_product_service, get_products_service};
use crate::utils::extract_user_id_from_cookie;
use mongodb::Database;
use validator::Validate;

pub async fn get_products_handler(
    req: HttpRequest,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let products = get_products_service(&db, &user_id_str).await?;

    let products_response: Vec<ProductResponse> =
        products.into_iter().map(ProductResponse::from).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": products_response,
        "code": 200
    })))
}

pub async fn post_product_handler(
    req: HttpRequest,
    payload: Result<Json<ProductDTO>, actix_web::Error>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let data = payload?.into_inner();
    data.validate()?;

    let product = create_product_service(data, &db, &user_id_str).await?;

    Ok(HttpResponse::Created().json({
        serde_json::json!({
            "status": "success",
            "data": ProductResponse::from(product),
            "code": 201
        })
    }))
}
