use crate::errors::ServiceError;
use crate::models::user::{CreateUserDTO, UpdateUserDTO, User};
use crate::utils::{handle_duplicate_key_error, hash_password, string_id_to_obj_id};
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

pub async fn get_user_service(id: &str, db: &Database) -> Result<User, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let object_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let filter = doc! { "_id": object_id };

    let user = collection
        .find_one(filter)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    user.ok_or_else(|| ServiceError::NotFound(format!("User dengan ID '{}' tidak ditemukan", id)))
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
        if username.len() < 6 {
            password = format!("{}123", username);
        } else {
            password = username.clone();
        }
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
            if let Some(conflict_error) = handle_duplicate_key_error(&err) {
                return Err(conflict_error);
            }

            Err(ServiceError::DatabaseError(err.to_string()))
        }
    }
}

pub async fn update_user_service(
    id: &str,
    payload: UpdateUserDTO,
    db: &Database,
) -> Result<User, ServiceError> {
    let object_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let mut update_doc = doc! {};

    if let Some(username) = payload.username {
        update_doc.insert("username", username);
    }
    if let Some(email) = payload.email {
        update_doc.insert("email", email);
    }
    if let Some(phone_number) = payload.phone_number {
        update_doc.insert("phone_number", phone_number);
    }

    if let Some(password) = payload.password {
        let hashed = hash_password(&password)
            .map_err(|e| ServiceError::HashingError(format!("Gagal hashing password: {}", e)))?;
        update_doc.insert("password_hash", hashed);
    }

    if update_doc.is_empty() {
        return Err(ServiceError::BadRequest(
            "Tidak ada data untuk di-update".to_string(),
        ));
    }

    let collection: Collection<User> = db.collection("users");

    collection
        .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
        .await
        .map_err(|err| {
            if let Some(conflict_error) = handle_duplicate_key_error(&err) {
                return conflict_error;
            }
            ServiceError::DatabaseError(err.to_string())
        })?;

    let updated_user = collection
        .find_one(doc! { "_id": object_id })
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("User tidak ditemukan!".to_string()))?;

    Ok(updated_user)
}

pub async fn delete_user_service(id: &str, db: &Database) -> Result<bool, ServiceError> {
    let object_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };

    let collection: Collection<User> = db.collection("users");

    let result = collection
        .delete_one(doc! {"_id": object_id})
        .await
        .map_err(|err| ServiceError::DatabaseError(err.to_string()))?;

    if result.deleted_count == 0 {
        return Err(ServiceError::NotFound("User tidak ditemukan!".into()));
    }

    Ok(true)
}
