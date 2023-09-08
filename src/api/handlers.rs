use crate::db::interfaces::DbConnectionWithMarket;
use crate::db::traits::MarketDatabase;
use weaver_core::api::errors::ApiErrorType;
use weaver_core::api::interfaces::{CFilterConnection, CacheConnection};
use weaver_core::api::responses::{json_serialize_embed, CallResponse, JsonReply};

/// Handles retrieving all listings from the database
pub async fn listings_handler(
    db: DbConnectionWithMarket,
    cache: CacheConnection,
    cf: CFilterConnection,
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
