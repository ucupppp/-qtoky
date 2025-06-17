use crate::errors::ServiceError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use bson::oid::ObjectId;
use mongodb::error::{Error, ErrorKind, WriteFailure};
use serde::Serializer;

pub fn object_id_as_string<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serializer.serialize_str(&oid.to_hex()),
        None => serializer.serialize_none(),
    }
}

pub fn string_id_to_obj_id(id: &str) -> Option<ObjectId> {
    ObjectId::parse_str(id).ok()
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash);
    if let Ok(parsed_hash) = parsed_hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else {
        false
    }
}

pub fn handle_duplicate_key_error(err: &Error) -> Option<ServiceError> {
    if let ErrorKind::Write(write_failure) = err.kind.as_ref() {
        match write_failure {
            WriteFailure::WriteError(write_error) => {
                if write_error.code == 11000 {
                    if let Some(field) = extract_duplicate_field(&write_error.message) {
                        return Some(ServiceError::Conflict(format!("{} sudah digunakan", field)));
                    }
                }
            }
            _ => {
                log::warn!("Write failure bukan WriteError, tidak diproses sebagai duplicate key.");
            }
        }
    }

    None
}

fn extract_duplicate_field(message: &str) -> Option<String> {
    let re = regex::Regex::new(r#"dup key: \{ ([^:]+):"#).ok()?;
    re.captures(message)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}
