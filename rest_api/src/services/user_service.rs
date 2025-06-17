use crate::models::user::User;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc, error::Error};

pub async fn get_users_service(db: &Database) -> Result<Vec<User>, Error> {
    let collection: Collection<User> = db.collection("users");

    let mut cursor = collection.find(doc! {}).await?;
    let mut users: Vec<User> = Vec::new();

    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }

    Ok(users)
}

