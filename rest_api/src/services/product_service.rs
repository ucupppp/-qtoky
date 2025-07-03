use crate::errors::ServiceError;
use crate::models::product::{Product, ProductDTO, UpdateProductDTO};
use crate::utils::{generate_random_sku, handle_duplicate_key_error, string_id_to_obj_id};
use bson::datetime::DateTime as BsonDateTime;

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
    payload: ProductDTO,
    db: &Database,
    id: &str,
) -> Result<Product, ServiceError> {
    let user_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let collection: Collection<Product> = db.collection("products");

    // Pakai SKU dari input, atau generate otomatis jika kosong

    let final_sku = match &payload.sku {
        Some(sku) if !sku.trim().is_empty() => sku.clone(),
        _ => generate_random_sku(),
    };

    let now = Utc::now();

    // Buat produk baru (sementara id None dulu)
    let mut product = Product {
        id: None,
        user_id: user_id,
        name: payload.name,
        sku: final_sku,
        price: payload.price,
        stock: payload.stock,
        category_id: payload
            .category_id
            .and_then(|id| ObjectId::parse_str(&id).ok()),
        created_at: Some(now),
        updated_at: Some(now),
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

pub async fn update_product_service(
    product_id: &str,
    payload: UpdateProductDTO,
    db: &Database,
    user_id: &str,
) -> Result<Product, ServiceError> {
    let object_id = match string_id_to_obj_id(product_id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid product ID".into())),
    };

    let user_object_id = match string_id_to_obj_id(user_id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid user ID".into())),
    };

    let mut update_doc = doc! {};

    if let Some(name) = payload.name {
        update_doc.insert("name", name);
    }
    if let Some(sku) = payload.sku {
        update_doc.insert("sku", sku);
    }
    if let Some(price) = payload.price {
        update_doc.insert("price", price);
    }
    if let Some(stock) = payload.stock {
        update_doc.insert("stock", stock);
    }
    if let Some(category_id) = payload.category_id {
        update_doc.insert("category_id", category_id);
    }

    if update_doc.is_empty() {
        return Err(ServiceError::BadRequest(
            "Tidak ada data untuk di-update".to_string(),
        ));
    }

    let now = Utc::now();
    update_doc.insert("updated_at", now.to_rfc3339());

    println!("{:?}", &update_doc);

    let collection: Collection<Product> = db.collection("products");

    // Tambahkan filter user_id di sini
    let filter = doc! {
        "_id": object_id,
        "user_id": user_object_id,
    };

    let update_result = collection
        .update_one(filter.clone(), doc! { "$set": update_doc })
        .await
        .map_err(|err| {
            if let Some(conflict_error) = handle_duplicate_key_error(&err) {
                return conflict_error;
            }
            ServiceError::DatabaseError(err.to_string())
        })?;

    println!("{:?}", &update_result);

    if update_result.matched_count == 0 {
        return Err(ServiceError::NotFound(
            "Produk tidak ditemukan atau tidak dimiliki oleh user ini".to_string(),
        ));
    }

    let updated_product = collection
        .find_one(filter)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            ServiceError::DatabaseError(e.to_string())
        })?
        .ok_or_else(|| ServiceError::NotFound("Produk tidak ditemukan!".to_string()))?;

    println!("{:?}", &updated_product);

    Ok(updated_product)
}
