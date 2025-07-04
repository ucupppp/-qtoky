use crate::utils::{object_id_as_string, opt_object_id_as_string};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_object_id_as_string"
    )]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub name: String,
    pub sku: String,
    pub price: u32,
    pub stock: u32,

    #[serde(serialize_with = "opt_object_id_as_string")]
    pub category_id: Option<ObjectId>,

    #[serde(default)]
    pub created_at: Option<bson::DateTime>,
    #[serde(default)]
    pub updated_at: Option<bson::DateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProductDTO {
    #[validate(length(min = 1, message = "Kolom name wajib diisi!"))]
    pub name: String,
    pub sku: Option<String>,

    #[validate(range(min = 100, message = "Harga minimal 100"))]
    pub price: u32,

    #[validate(range(max = 99999, message = "Stok maksimal 99999"))]
    pub stock: u32,

    // Optional: kategori (boleh kosong)
    pub category_id: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductDTO {
    #[validate(length(min = 1, message = "Kolom name tidak boleh kosong"))]
    pub name: Option<String>,

    pub sku: Option<String>,

    #[validate(range(max = 99999, message = "Stok maksimal 99999"))]
    pub stock: Option<u32>,

    #[validate(range(min = 100, message = "Harga minimal 100"))]
    pub price: Option<u32>,

    pub category_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,

    pub user_id: String,

    pub name: String,
    pub sku: String,
    pub price: u32,
    pub stock: u32,

    pub category_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        ProductResponse {
            id: p
                .id
                .expect("Product.id harus ada setelah input data")
                .to_hex(),
            user_id: p.user_id.to_hex(),
            name: p.name,
            sku: p.sku,
            price: p.price,
            stock: p.stock,
            category_id: p.category_id.map(|c| c.to_hex()),
            created_at: p.created_at.map(|t| t.to_chrono().to_rfc3339()),
            updated_at: p.updated_at.map(|t| t.to_chrono().to_rfc3339()),
        }
    }
}
