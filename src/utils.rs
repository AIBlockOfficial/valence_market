use crate::constants::{DRUID_CHARSET, DRUID_LENGTH};
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use std::str::FromStr;

/// Constructs a 16 byte DRUID string
pub fn construct_druid() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..DRUID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..DRUID_CHARSET.len());
            DRUID_CHARSET[idx] as char
        })
        .collect();

    random_string
}

pub fn construct_mongodb_object_id(id: String) -> ObjectId {
    match ObjectId::from_str(&id) {
        Ok(object_id) => object_id,
        Err(_) => ObjectId::new(),
    }
}