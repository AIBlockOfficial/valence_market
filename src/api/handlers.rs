use crate::db::interfaces::DbConnectionWithMarket;
use crate::db::traits::MarketDatabase;
use crate::db::interfaces::Listing;
use weaver_core::api::errors::ApiErrorType;
use weaver_core::api::interfaces::{CFilterConnection, CacheConnection};
use weaver_core::api::responses::{json_serialize_embed, CallResponse, JsonReply};

/// Handles retrieving all listings
/// 
/// ### Arguments
/// 
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
pub async fn listings_handler(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listings");
    let db_lock = db.lock().await;

    let listings = match db_lock.get_listings().await {
        Ok(listings) => listings,
        Err(e) => {
            return r.into_err_internal(ApiErrorType::DBInsertionFailed);
        }
    };

    r.into_ok(
        "Data retrieved successfully",
        json_serialize_embed(listings),
    )
}

/// Handles adding a listing to the database
/// 
/// ### Arguments
/// 
/// * `payload` - The listing to add
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub async fn listing_send_handler(
    payload: Listing,
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listing_send");
    let mut db_lock = db.lock().await;

    match db_lock.add_listing(payload.clone()).await {
        Ok(_) => {
            r.into_ok("Listing added successfully", json_serialize_embed(payload))
        }
        Err(_) => {
            r.into_err_internal(ApiErrorType::DBInsertionFailed)
        }
    }
}
