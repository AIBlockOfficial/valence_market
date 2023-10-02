use crate::constants::{DRUID_CHARSET, DRUID_LENGTH};
use crate::market::interfaces::{Order, OrderBook};
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use std::str::FromStr;
use chrono::prelude::Utc;

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

/// Constructs an initial orderbook for a new listing
/// 
/// ### Arguments
/// 
/// * `listing_id` - The ID of the listing to create the orderbook for
/// * `price` - The price of the initial ask order
/// * `quantity` - The quantity of the initial ask order
/// * `desired_listing_id` - The ID of the listing asset to trade the initial order with (optional)
pub fn construct_initial_orderbook(listing_id: String, price: f64, quantity: f64, desired_listing_id: Option<String>) -> OrderBook {
    let init_order = construct_initial_order(listing_id, price, quantity, desired_listing_id);
    let asks = vec![init_order];

    OrderBook {
        asks,
        bids: Vec::new(),
        pending_trades: Vec::new()
    }
}

/// Constructs an initial order for a new listing
/// 
/// ### Arguments
/// 
/// * `listing_id` - The ID of the listing to create the order for
/// * `price` - The price of the order
/// * `quantity` - The quantity of the order
/// * `desired_listing_id` - The ID of the listing to trade with (optional)
fn construct_initial_order(listing_id: String, price: f64, quantity: f64, desired_listing_id: Option<String>) -> Order {
    // We can use the same function to get a base order ID as for a DRUID
    let id = construct_druid();

    Order {
        id,
        listing_id,
        price,
        quantity,
        is_bid: false,
        created_at: Utc::now().to_string(),
        druid: None,
        desired_listing_id,
    }
}
