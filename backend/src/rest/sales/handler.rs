use crate::models::sale::{Sale, SaleItem, SaleDTO, SaleResponse};
use actix_web::{
    Error as ActixError, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path},
};

use crate::errors::ApiError;
use crate::models::product::{ProductDTO, ProductResponse, UpdateProductDTO};
use crate::services::sale_service::{create_sale_service, get_sales_service};
use crate::utils::extract_user_id_from_cookie;
use mongodb::Database;
use validator::Validate;



pub async fn post_sale_handler(
    req: HttpRequest,
    payload: Result<Json<SaleDTO>, ActixError>,
    db: Data<Database>,
) -> Result<HttpResponse, ApiError> {
    println!("LOGG{:?}", &payload);
    let user_id_str = extract_user_id_from_cookie(&req)?;
    let data = payload?.into_inner();
    data.validate()?;

    let sale = create_sale_service(data, &db, &user_id_str).await?;

    Ok(HttpResponse::Created().json({
        serde_json::json!({
            "status" : "success",
            "data" : SaleResponse::from(sale),
            "code" : 201
        })
    }))
}
