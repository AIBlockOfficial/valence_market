use crate::market::interfaces::OrderBook;
use futures::lock::Mutex;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use weaver_core::db::mongo_db::MongoDbConn;

/// A MongoDB document wrapper for an asset listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDbOrderBook {
    pub _id: ObjectId,
    pub order_book: OrderBook,
}

/// Trait wrapper struct for a MongoDB connection that stores market data
#[derive(Debug, Clone)]
pub struct MongoDbConnWithMarket {
    pub inner: Arc<Mutex<MongoDbConn>>,
}

impl MongoDbConnWithMarket {
    /// Creates a new MongoDbConnWithMarket
    ///
    /// ### Arguments
    ///
    /// * `inner` - The MongoDB connection to wrap
    pub fn new(inner: Arc<Mutex<MongoDbConn>>) -> Self {
        Self { inner }
    }
}
