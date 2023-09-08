use crate::db::interfaces::DbConnectionWithMarket;
use crate::db::interfaces::Listing;
use crate::db::traits::MarketDatabase;
use crate::market::interfaces::Order;
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
        Ok(_) => r.into_ok("Listing added successfully", json_serialize_embed(payload)),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}

/// Handles retrieving a listing by its ID
///
/// ### Arguments
///
/// * `id` - The ID of the listing to retrieve
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub async fn listing_by_id_handler(
    id: String,
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listing_by_id");
    let db_lock = db.lock().await;

    match db_lock.get_listing_by_id(id).await {
        Ok(listing) => r.into_ok(
            "Listing retrieved successfully",
            json_serialize_embed(listing),
        ),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}

/// Handles retrieving orders by their listing ID
/// 
/// ### Arguments
/// 
/// * `id` - The ID of the listing to retrieve orders for
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub async fn orders_by_id_handler(
    id: String,
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("orders_by_id");
    let db_lock = db.lock().await;

    match db_lock.get_orders_by_id(id).await {
        Ok(orders) => r.into_ok(
            "Orders retrieved successfully",
            json_serialize_embed(orders),
        ),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}

/// Handles adding an order to the database
/// 
/// ### Arguments
/// 
/// * `payload` - The order to add
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub async fn orders_send_handler(
    payload: Order,
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("orders_send");
    let mut db_lock = db.lock().await;

    match db_lock.add_order(payload.clone()).await {
        Ok(_) => r.into_ok("Order added successfully", json_serialize_embed(payload)),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}
