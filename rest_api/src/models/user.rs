use crate::utils::object_id_as_string;
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
    pub password_hash: String,

    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDTO {
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.unwrap_or_default().to_hex(),
            username: user.username,
            email: user.email,
            phone_number: user.phone_number,
        }
    }
}

impl From<RegisterDTO> for User {
    fn from(dto: RegisterDTO) -> Self {
        User {
            id: None,
            username: dto.username,
            email: dto.email,
            password_hash: String::new(), // nanti diisi setelah hash password
            phone_number: dto.phone_number,
        }
    }
}

