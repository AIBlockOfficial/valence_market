use crate::db::interfaces::MongoDbConnWithMarket;
use crate::market::interfaces::{Listing, Order};
use crate::db::traits::MarketDatabase;
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
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listings");
    
    let listings = match db.get_listings().await {
        Ok(listings) => listings,
        Err(_e) => {
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
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
    _cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listing_send");
    
    match db.add_listing(payload.clone()).await {
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
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
    _cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("listing_by_id");
    
    match db.get_listing_by_id(id).await {
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
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
    _cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("orders_by_id");
    
    match db.get_orders_by_id(id).await {
        Ok(orders) => r.into_ok(
            "Orders retrieved successfully",
            json_serialize_embed(orders),
        ),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}

/// Handles retrieving pending trades by their listing ID
/// 
/// ### Arguments
/// 
/// * `id` - The ID of the listing to retrieve pending trades for
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub async fn orders_pending_handler(
    id: String,
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
    _cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("orders_pending");
    
    match db.get_pending_trades_by_id(id).await {
        Ok(pending_trades) => r.into_ok(
            "Pending trades retrieved successfully",
            json_serialize_embed(pending_trades),
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
    db: MongoDbConnWithMarket,
    _cache: CacheConnection,
    _cf: CFilterConnection,
) -> Result<JsonReply, JsonReply> {
    let r = CallResponse::new("orders_send");
    
    match db.add_order(payload.clone()).await {
        Ok(_) => r.into_ok("Order added successfully", json_serialize_embed(payload)),
        Err(_) => r.into_err_internal(ApiErrorType::DBInsertionFailed),
    }
}
