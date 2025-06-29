use crate::errors::ServiceError;
use crate::models::product::{Product, ProductDTO};
use crate::utils::{generate_random_sku, handle_duplicate_key_error, string_id_to_obj_id};

use bson::oid::ObjectId;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get_products_service(db: &Database, id: &str) -> Result<Vec<Product>, ServiceError> {
    let user_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };
    let collection: Collection<Product> = db.collection("products");

    let mut cursor = collection
        .find(doc! {"user_id": user_id})
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    let mut users: Vec<Product> = Vec::new();

    while let Some(user) = cursor
        .try_next()
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
    {
        users.push(user);
    }

    Ok(users)
}

pub async fn create_product_service(
    dto: ProductDTO,
    db: &Database,
    id: &str,
) -> Result<Product, ServiceError> {
    let user_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let collection: Collection<Product> = db.collection("products");

    // Pakai SKU dari input, atau generate otomatis jika kosong

    let final_sku = match &dto.sku {
        Some(sku) if !sku.trim().is_empty() => sku.clone(),
        _ => generate_random_sku(),
    };

    let now = Utc::now();

    // Buat produk baru (sementara id None dulu)
    let mut product = Product {
        id: None,
        user_id: user_id,
        name: dto.name,
        sku: final_sku,
        price: dto.price,
        stock: dto.stock,
        category_id: dto.category_id.and_then(|id| ObjectId::parse_str(&id).ok()),
        created_at: Some(now),
    };

    // Insert ke database
    let result = collection.insert_one(&product).await;

    match result {
        Ok(insert_result) => {
            product.id = insert_result
                .inserted_id
                .as_object_id()
                .map(|oid| oid.to_owned());
            Ok(product)
        }
        Err(e) => {
            if let Some(err) = handle_duplicate_key_error(&e) {
                return Err(err);
            }
            Err(ServiceError::DatabaseError(e.to_string()))
        }
    }
}
