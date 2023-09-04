use beacon_core::api::interfaces::{CFilterConnection, CacheConnection, DbConnection};
use beacon_core::api::utils::{
    handle_rejection, map_api_res, post_cors, with_node_component,
};
use warp::{Filter, Rejection, Reply};

/// ========== BASE ROUTES ========== ///

pub fn listings(
    db: DbConnection,
    cache: CacheConnection,
    cuckoo_filter: CFilterConnection,
    body_limit: u64,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("listings")
        .and(warp::body::content_length_limit(body_limit))
        .and(warp::body::json())
        .and(with_node_component(cache))
        .and(with_node_component(db))
        .and(with_node_component(cuckoo_filter))
        .and_then(move |_, data, cache, db, cf| map_api_res(get_data_handler(db, cache, data, cf)))
        .recover(handle_rejection)
        .with(post_cors())
}