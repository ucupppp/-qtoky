use actix_web::{
    Error as ActixError, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path},
};

use crate::errors::ApiError;
use crate::models::product::{ProductDTO, ProductResponse, UpdateProductDTO};
use crate::services::product_service::{
    create_product_service, delete_product_service, get_product_service, get_products_service,
    update_product_service,
};
use crate::utils::extract_user_id_from_cookie;
use mongodb::Database;
use validator::Validate;

pub async fn get_products_handler(
    req: HttpRequest,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    println!("{:?}", req);
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

pub async fn get_product_handler(
    req: HttpRequest,
    path: Path<String>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let product_id = path.into_inner();
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let product = get_product_service(&product_id, &db, &user_id_str).await?;

    let product_response: ProductResponse = product.into(); // konversi eksplisit dulu

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": product_response,
        "code" : 200
    })))
}
pub async fn post_product_handler(
    req: HttpRequest,
    payload: Result<Json<ProductDTO>, ActixError>,
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

pub async fn patch_product_handler(
    req: HttpRequest,
    payload: Result<Json<UpdateProductDTO>, ActixError>,
    db: Data<Database>,
    path: Path<String>,
) -> Result<HttpResponse, ApiError> {
    let product_id = path.into_inner();
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let data = payload?.into_inner();
    data.validate()?;
    let product = update_product_service(&product_id, data, &db, &user_id_str).await?;

    Ok(HttpResponse::Ok().json({
        serde_json::json!({
            "status": "success",
            "data": ProductResponse::from(product),
            "code": 200
        })
    }))
}

pub async fn delete_product_handler(
    req: HttpRequest,
    db: Data<Database>,
    path: Path<String>,
) -> Result<HttpResponse, ApiError> {
    let product_id = path.into_inner();
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let _delete_product = delete_product_service(&product_id, &db, &user_id_str).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "code": 204
    })))
}
