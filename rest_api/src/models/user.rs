use crate::utils::object_id_as_string;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDTO {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: String,
    #[validate(email(message = "Email tidak valid"))]
    pub email: String,
    #[validate(length(min = 10, message = "Nomor HP minimal 10 digit"))]
    pub phone_number: Option<String>,
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: String,
    #[validate(email(message = "Email tidak valid"))]
    pub email: String,
    #[validate(length(min = 10, message = "Nomor HP minimal 10 digit"))]
    pub phone_number: Option<String>,
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: Option<String>,

    #[validate(email(message = "Email tidak valid"))]
    pub email: Option<String>,

    #[validate(length(min = 10, message = "Nomor HP minimal 10 digit"))]
    pub phone_number: Option<String>,

    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
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
