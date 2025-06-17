use crate::utils::function::object_id_as_string;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "object_id_as_string"
    )]
    pub id: Option<ObjectId>,

    pub username: String,
    pub email: String,

    #[serde(skip_serializing)] // agar tidak ikut dikirim saat response
    pub password_hash: String,

    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

impl From<Register> for User {
    fn from(register: Register) -> Self {
        User {
            id: None,
            username: register.username,
            email: register.email,
            password_hash: String::new(), // nanti isi setelah hash
            phone_number: register.phone_number,
        }
    }
}

