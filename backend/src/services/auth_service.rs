use crate::errors::ServiceError;
use crate::models::user::{LoginDTO, RegisterDTO, User};
use crate::utils::{
    handle_duplicate_key_error, hash_password, string_id_to_obj_id, verify_password,
};
use mongodb::{Collection, Database, bson::doc};

pub async fn login_service(payload: LoginDTO, db: &Database) -> Result<User, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let user = collection
        .find_one(doc! { "username": &payload.username })
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(ServiceError::Unauthorized(
                "username atau password salah".into(),
            ));
        }
    };

    if !verify_password(&payload.password, &user.password_hash) {
        return Err(ServiceError::Unauthorized(
            "username atau password salah".into(),
        ));
    }

    Ok(user)
}

pub async fn register_service(payload: RegisterDTO, db: &Database) -> Result<User, ServiceError> {
    let collection: Collection<User> = db.collection("users");

    let RegisterDTO {
        username,
        email,
        phone_number,
        password,
    } = payload;

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
