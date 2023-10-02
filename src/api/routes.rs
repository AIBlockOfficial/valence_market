use crate::api::handlers::{
    listing_by_id_handler, listing_send_handler, listings_handler, orders_by_id_handler,
    orders_pending_handler, orders_send_handler,
};
use crate::db::traits::MarketDatabase;
use crate::market::interfaces::Listing;
use futures::lock::Mutex;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use weaver_core::api::interfaces::CFilterConnection;
use weaver_core::api::utils::{get_cors, map_api_res, post_cors, with_node_component};
use weaver_core::db::handler::KvStoreConnection;

// ========== LISTING ROUTES ========== //

/// GET /listings
///
/// Retrieves all listings from the database
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub fn listings<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::get())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and_then(move |cache, db| map_api_res(listings_handler(db, cache)))
        .with(get_cors())
}

/// GET /listings/{id}
///
/// Retrieves a listing from the database by its ID
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub fn listing_by_id<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
    cuckoo_filter: CFilterConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |id, cache, db, cf| map_api_res(listing_by_id_handler(id, db, cache, cf)))
        .with(get_cors())
}

/// POST /listings/send
///
/// Adds a listing to the database
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `body_limit` - The maximum size of the request body
pub fn listing_send<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
    body_limit: u64,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::post())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and_then(move |data: Listing, cache, db| {
            map_api_res(listing_send_handler(data, db, cache))
        })
        .with(post_cors())
}

// ========== ORDER ROUTES ========== //

/// GET /orders/{id}
///
/// Retrieves an order from the database by its listing ID
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
pub fn orders_by_id<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
    cuckoo_filter: CFilterConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("orders")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |id, cache, db, cf| map_api_res(orders_by_id_handler(id, db, cache, cf)))
        .with(get_cors())
}

/// GET /orders/pending/{id}
///
/// Retrieves all pending trades for a listing from the database by its ID
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cf` - The cuckoo filter connection to use
pub fn orders_pending<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
    cuckoo_filter: CFilterConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("orders" / "pending")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_node_component(db))
        .and(with_node_component(cache))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |id, db, cache, cf| map_api_res(orders_pending_handler(id, db, cache, cf)))
        .with(get_cors())
}

/// POST /orders/send
///
/// Adds an open order to a listing
///
/// ### Arguments
///
/// * `db` - The database connection to use
/// * `cache` - The cache connection to use
/// * `cuckoo_filter` - The cuckoo filter connection to use
/// * `body_limit` - The maximum size of the request body
pub fn orders_send<
    D: MarketDatabase + Clone + Send + Sync + 'static,
    C: KvStoreConnection + Clone + Send + Sync + 'static,
>(
    db: Arc<Mutex<D>>,
    cache: Arc<Mutex<C>>,
    cuckoo_filter: CFilterConnection,
    body_limit: u64,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("orders")
        .and(warp::post())
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |data, cache, db, cf| map_api_res(orders_send_handler(data, db, cache, cf)))
        .with(post_cors())
}
