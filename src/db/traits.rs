use crate::constants::{MARKET_COLL_NAME, MARKET_DB_NAME};
use crate::db::interfaces::{Listing, MongoDbConnWithMarket};
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson};
use mongodb::Collection;
use weaver_core::api::errors::{construct_result_error, ApiError};

//====== TRAIT IMPLEMENTATIONS ======//

/// Trait for a database that stores market data
#[async_trait]
pub trait MarketDatabase {
    /// Gets all listings from the database
    async fn get_listings(&self) -> Result<Vec<Listing>, ApiError>;

    /// Adds a listing to the database
    ///
    /// ### Arguments
    ///
    /// * `listing` - The listing to add
    async fn add_listing(&mut self, listing: Listing) -> Result<(), ApiError>;
}

#[async_trait]
impl MarketDatabase for MongoDbConnWithMarket {
    async fn get_listings(&self) -> Result<Vec<Listing>, ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection: Collection<Listing> = db.collection(&MARKET_COLL_NAME);
        let mut asset_listings: Vec<Listing> = Vec::new();

        // Define a filter (empty document) to retrieve all documents in the collection
        let filter = doc! {};

        // Find all documents in the collection and deserialize them into Listing objects
        let mut cursor = match collection.find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't fetch documents from DB",
                    "listings",
                ));
            }
        };

        while let Ok(true) = cursor.advance().await {
            let listing: Listing = match cursor.deserialize_current() {
                Ok(listing) => listing,
                Err(_) => {
                    return Err(construct_result_error(
                        "Couldn't deserialize listing",
                        "listings",
                    ));
                }
            };

            asset_listings.push(listing);
        }

        Ok(asset_listings)
    }

    async fn add_listing(&mut self, listing: Listing) -> Result<(), ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection = db.collection(MARKET_COLL_NAME);

        // Serialize the new listing to BSON
        let bson_new_listing = match to_bson(&listing) {
            Ok(bson) => bson,
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't serialize listing for storage",
                    "listings",
                ));
            }
        };

        // Insert the BSON document into the collection
        match collection.insert_one(bson_new_listing, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(construct_result_error(
                "Couldn't insert listing into DB",
                "listings",
            )),
        }
    }
}
