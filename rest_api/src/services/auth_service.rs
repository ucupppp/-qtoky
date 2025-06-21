use crate::errors::ServiceError;
use crate::models::user::{LoginDTO, User};
use crate::utils::verify_password;
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

