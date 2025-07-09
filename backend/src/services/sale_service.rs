use crate::errors::ServiceError;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};


pub async fn get_sales_service(db: &Database, id:&str) -> Result<Sale, ServiceError>{
    let user_id = match string_id_to_obj_id(id) {
        Some(oid) => oid,
        None => return Err(ServiceError::InvalidId("Invalid ID".into())),
    };
    let collection: Collection<Sale> = db.collection("sales");

    let mut cursor = collection
        .find(doc! {"user_id":user_id})
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
    
    let mut sales: Vec<Product> = Vec::new();

    while let Some(sales) = cursor
        .try_next()
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
    {
        products.push(sales);
    }

    Ok(sales)
}
