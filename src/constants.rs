/// ==== DRUID ==== ///

pub const DRUID_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

pub const DRUID_LENGTH: usize = 16;

/// ==== DATABASE ==== ///

pub const MARKET_DB_NAME: &str = "market";
pub const MARKET_COLL_NAME: &str = "listings";
pub const MARKET_COLL_NAME_ORDERS: &str = "orders";
