use bson::oid::ObjectId;
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
