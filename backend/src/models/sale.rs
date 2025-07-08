use crate::utils::{object_id_as_string, opt_object_id_as_string};
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleItem {
    #[serde(serialize_with = "object_id_as_string")]
    pub product_id: ObjectId,
    pub product_name: String,
    pub sku: String,
    pub quantity: i32,
    pub price: f64,
    pub discount: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sale {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_object_id_as_string"
    )]
    pub id: Option<ObjectId>,

    #[serde(serialize_with = "object_id_as_string")]
    pub user_id: ObjectId,
    pub customer_id: Option<ObjectId>,
    pub items: Vec<SaleItem>,

    pub total_amount: f64,
    pub discount_total: f64,
    pub final_amount: f64,

    pub paid_amount: f64,
    pub remaining_amount: f64,
    pub status: String, // "paid", "partial", "unpaid"

    pub invoice_number: Option<String>,

    #[serde(serialize_with = "opt_object_id_as_string")]
    pub payment_method_id: Option<ObjectId>,
    pub sale_date: DateTime,
    pub notes: Option<String>,

    #[serde(default)]
    pub created_at: Option<DateTime>,
    #[serde(default)]
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SaleItemDTO {
    pub product_id: ObjectId, // tidak divalidasi karena sudah pasti BSON ID yang valid

    #[validate(range(min = 1, message = "Jumlah item minimal 1"))]
    pub quantity: i32,

    #[validate(range(min = 100.0, message = "Harga minimal 100"))]
    pub price: f64,

    #[validate(range(min = 0.0, message = "Diskon tidak boleh negatif"))]
    pub discount: Option<f64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SaleDTO {
    pub customer_id: Option<ObjectId>,

    #[validate(length(min = 1, message = "Daftar item tidak boleh kosong"))]
    #[validate(nested)]
    pub items: Vec<SaleItemDTO>,

    pub payment_method_id: Option<ObjectId>,

    #[validate(range(min = 0.0, message = "Jumlah bayar tidak boleh negatif"))]
    pub paid_amount: f64,

    #[validate(length(min = 0, max = 255, message = "Catatan maksimal 255 karakter"))]
    pub notes: Option<String>,
}
