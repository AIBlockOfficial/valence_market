use crate::constants::{MARKET_COLL_NAME, MARKET_DB_NAME, MARKET_COLL_NAME_ORDERS};
use crate::db::interfaces::{Listing, MongoDbConnWithMarket, MongoDbOrderBook};
use crate::utils::construct_mongodb_object_id;
use crate::market::interfaces::{Order, OrderBook, PendingTrade};
use async_trait::async_trait;
use mongodb::bson::doc;
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

    /// Gets a listing from the database by its ID
    /// 
    /// ### Arguments
    /// 
    /// * `id` - The ID of the listing to retrieve
    async fn get_listing_by_id(&self, id: String) -> Result<Listing, ApiError>;

    /// Gets the orderbook for a listing from the database by its ID
    ///
    /// ### Arguments
    /// 
    /// * `id` - The ID of the listing to retrieve
    async fn get_orders_by_id(&self, id: String) -> Result<OrderBook, ApiError>;

    /// Adds an order to the orderbook for a listing
    /// 
    /// ### Arguments
    /// 
    /// * `id` - The ID of the listing to add the order to
    /// * `order` - The order to add
    async fn add_order(&mut self, order: Order) -> Result<(), ApiError>;

    /// Gets all pending trades for a listing from the database by its ID
    /// 
    /// ### Arguments
    /// 
    /// * `id` - The ID of the listing to retrieve
    async fn get_pending_trades_by_id(&self, id: String) -> Result<Vec<PendingTrade>, ApiError>;
    
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
        let ob_id = listing._id.clone();

        // Insert the BSON document into the collection
        let _listing_addition = match collection.insert_one(listing, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(construct_result_error(
                "Couldn't insert listing into DB",
                "listings",
            )),
        };

        // Create a new orders collection for the listing
        let orders_collection = db.collection(MARKET_COLL_NAME_ORDERS);
        let new_orderbook = MongoDbOrderBook {
            _id: ob_id,
            order_book: Default::default(),
        };

        // Insert the orderbook into the collection
        match orders_collection.insert_one(new_orderbook, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(construct_result_error(
                "Couldn't insert listing into DB",
                "listings",
            )),
        }
    }

    async fn get_listing_by_id(&self, id: String) -> Result<Listing, ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection: Collection<Listing> = db.collection(&MARKET_COLL_NAME);
        let filter = doc! { "_id": construct_mongodb_object_id(id) };

        // Retrieve the listing from the database using the filter
        match collection.find_one(filter, None).await {
            Ok(listing) => match listing {
                Some(listing) => Ok(listing),
                None => {
                    return Err(construct_result_error(
                        "Couldn't find listing with given ID",
                        "listings",
                    ));
                }
            },
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't fetch listing from DB",
                    "listings",
                ));
            }
        }
    }

    async fn get_orders_by_id(&self, id: String) -> Result<OrderBook, ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection: Collection<MongoDbOrderBook> = db.collection(&MARKET_COLL_NAME_ORDERS);
        let filter = doc! { "_id": construct_mongodb_object_id(id) };

        // Retrieve the orderbook from the database using the filter
        match collection.find_one(filter, None).await {
            Ok(ob) => match ob {
                Some(ob) => Ok(ob.order_book),
                None => {
                    return Err(construct_result_error(
                        "Couldn't find orderbook with given ID",
                        "listings",
                    ));
                }
            },
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't fetch orderbook from DB",
                    "listings",
                ));
            }
        }
    }

    async fn add_order(&mut self, order: Order) -> Result<(), ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection: Collection<MongoDbOrderBook> = db.collection(&MARKET_COLL_NAME_ORDERS);
        let filter = doc! { "_id": construct_mongodb_object_id(order.asset_address.clone()) };

        // Retrieve the orderbook from the database using the filter
        match collection.find_one(filter.clone(), None).await {
            Ok(ob) => match ob {
                Some(mut ob) => {
                    let address = order.asset_address.clone();

                    ob.order_book.add_order(order);
                    let new_orderbook = MongoDbOrderBook {
                        _id: construct_mongodb_object_id(address),
                        order_book: ob.order_book,
                    };

                    // Insert the BSON document into the collection
                    match collection.replace_one(filter, new_orderbook, None).await {
                        Ok(_) => Ok(()),
                        Err(_) => Err(construct_result_error(
                            "Couldn't insert orderbook into DB",
                            "listings",
                        )),
                    }
                },
                None => {
                    return Err(construct_result_error(
                        "Couldn't find orderbook with given ID",
                        "listings",
                    ));
                }
            },
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't fetch orderbook from DB",
                    "listings",
                ));
            }
        }
    }

    async fn get_pending_trades_by_id(&self, id: String) -> Result<Vec<PendingTrade>, ApiError> {
        let db = self.inner.client.database(MARKET_DB_NAME);
        let collection: Collection<MongoDbOrderBook> = db.collection(&MARKET_COLL_NAME_ORDERS);
        let filter = doc! { "_id": construct_mongodb_object_id(id) };

        // Retrieve the orderbook from the database using the filter
        match collection.find_one(filter, None).await {
            Ok(ob) => match ob {
                Some(ob) => Ok(ob.order_book.pending_trades),
                None => {
                    return Err(construct_result_error(
                        "Couldn't find orderbook with given ID",
                        "listings",
                    ));
                }
            },
            Err(_) => {
                return Err(construct_result_error(
                    "Couldn't fetch orderbook from DB",
                    "listings",
                ));
            }
        }
    }
}
