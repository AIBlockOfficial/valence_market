use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use weaver_core::db::mongo_db::MongoDbConn;

/// An asset listing on the market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
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
