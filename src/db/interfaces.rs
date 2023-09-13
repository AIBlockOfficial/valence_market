use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use weaver_core::db::mongo_db::MongoDbConn;
use mongodb::bson::oid::ObjectId;
use crate::market::interfaces::OrderBook;

/// An asset listing on the market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub price: f64,
}

/// A MongoDB document wrapper for an asset listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDbOrderBook {
    pub _id: ObjectId,
    pub order_book: OrderBook
}

/// Trait wrapper struct for a MongoDB connection that stores market data
#[derive(Debug, Clone)]
pub struct MongoDbConnWithMarket {
    pub inner: MongoDbConn,
}

impl MongoDbConnWithMarket {
    /// Creates a new MongoDbConnWithMarket
    ///
    /// ### Arguments
    ///
    /// * `inner` - The MongoDB connection to wrap
    pub fn new(inner: MongoDbConn) -> Self {
        Self { inner }
    }
}

pub type DbConnectionWithMarket = Arc<Mutex<MongoDbConnWithMarket>>;
