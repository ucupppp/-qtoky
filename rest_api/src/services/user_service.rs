use crate::errors::ServiceError;
use crate::models::user::{CreateUserDTO, User};
use crate::utils::{hash_password, string_id_to_obj_id};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get_users_service(db: &Database) -> Result<Vec<User>, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let mut cursor = collection
        .find(doc! {})
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    let mut users: Vec<User> = Vec::new();

    while let Some(user) = cursor
        .try_next()
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
    {
        users.push(user);
    }

    Ok(users)
}

pub async fn get_user_service(user_id: &str, db: &Database) -> Result<User, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let object_id = match string_id_to_obj_id(user_id) {
        Some(id) => id,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let filter = doc! { "_id": object_id };

    let user = collection
        .find_one(filter)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    user.ok_or_else(|| {
        ServiceError::NotFound(format!("User dengan ID '{}' tidak ditemukan", user_id))
    })
}

pub async fn create_user_service(
    payload: CreateUserDTO,
    db: &Database,
) -> Result<User, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let username = payload.username;
    let email = payload.email;
    let phone_number = payload.phone_number;
    let mut password = payload.password.unwrap_or_else(|| "".to_string());

    if password.trim().is_empty() {
        password = username.clone();
    }

    let hashed_password = hash_password(&password)
        .map_err(|e| ServiceError::HashingError(format!("Gagal hashing password: {}", e)))?;

    let new_user = User {
        id: None,
        username,
        email,
        password_hash: hashed_password,
        phone_number,
    };

    let result = collection.insert_one(&new_user).await;

    match result {
        Ok(_) => Ok(new_user),
        Err(err) => {
            if err.to_string().contains("E11000") {
                return Err(ServiceError::Conflict(
                    "Username atau Email sudah digunakan".to_string(),
                ));
            }
            Err(ServiceError::DatabaseError(err.to_string()))
        }
    }
}
