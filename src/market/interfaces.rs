use crate::utils::construct_druid;
use serde::{ Deserialize, Serialize };
use mongodb::bson::oid::ObjectId;

/// An asset listing on the market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub initial_price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PendingTrade {
    pub bid_id: String,
    pub ask_id: String,
    pub quantity: f64,
    pub price: f64,
    pub created_at: String,
    pub druid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    pub id: String,
    pub listing_id: String,
    pub price: f64,
    pub quantity: f64,
    pub is_bid: bool,
    pub created_at: String,
    pub druid: Option<String>,
    pub desired_listing_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub pending_trades: Vec<PendingTrade>,
}

/// Finds the index for an order to be inserted at based on the price
///
/// ### Arguments
///
/// * `prices` - A list of current orders
/// * `price` - The price of the order to be inserted
pub fn find_index_for_order(prices: &mut Vec<Order>, price: &f64) -> usize {
    // If there are no orders, return 0
    if prices.len() == 0 {
        return 0;
    }

    let mut left = 0;
    let mut right = prices.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_price = &prices[mid].price;

        if mid_price == price {
            return mid;
        } else if mid_price < price {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    left
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: Vec::new(),
            asks: Vec::new(),
            pending_trades: Vec::new(),
        }
    }

    /// Matches an order with the lowest ask/highest bid, if possible. If not possible,
    /// the order is added to the order book
    ///
    /// ### Arguments
    ///
    /// * `order` - The order to be matched
    pub fn add_order(&mut self, order: &mut Order) {
        let match_list = if order.is_bid { &mut self.asks } else { &mut self.bids };
        let mut empty_orders = vec![Vec::<usize>::new(), Vec::<usize>::new()];
        let mut match_idx = 0;

        // Base case, where we're dealing with an empty orderbook
        if match_list.len() == 0 {
            self.insert_order_in_list(order.clone());
            return;
        }

        // The match order/list is the opposite side of the trade from the order.
        // If the order is a bid, then the match order will be an ask
        while match_idx < match_list.len() && order.quantity > 0.0 {
            let match_order = &match_list[match_idx];

            if
                (order.is_bid && match_order.price <= order.price) ||
                (!order.is_bid && match_order.price >= order.price)
            {
                let quantity = match_order.quantity.min(order.quantity);
                let bid_id = if order.is_bid { order.id.clone() } else { match_order.id.clone() };
                let ask_id = if !order.is_bid { order.id.clone() } else { match_order.id.clone() };
                let pending_trade = PendingTrade {
                    bid_id,
                    ask_id,
                    quantity,
                    price: match_order.price.min(order.price),
                    created_at: String::from(""),
                    druid: construct_druid(),
                };

                // Handle pending trades and current orders
                self.pending_trades.push(pending_trade);
                match_list[match_idx].quantity -= quantity;
                order.quantity -= quantity;

                // Add empty orders to the list for later clean up
                if match_list[match_idx].quantity == 0.0 {
                    if order.is_bid {
                        empty_orders[0].push(match_idx);
                    } else {
                        empty_orders[1].push(match_idx);
                    }
                }

                // Carry on to the next order
                match_idx += 1;

            } else {
                self.insert_order_in_list(order.clone());
                break;
            }
        }

        self.clean_up_empty_orders(empty_orders);
    }

    /// Inserts an order into the order book at the correct index
    ///
    /// ### Arguments
    ///
    /// * `order` - The order to be inserted
    fn insert_order_in_list(&mut self, order: Order) {
        let order_list = if order.is_bid { &mut self.bids } else { &mut self.asks };
        let search_idx = find_index_for_order(order_list, &order.price);

        let idx = match search_idx {
            0 => 0,
            _ => if order_list[search_idx].price > order.price && order.is_bid {
                search_idx + 1
            } else {
                search_idx - 1
            }
        };

        order_list.insert(idx, order);
    }

    /// Removes orders from the order book if they have no quantity left
    ///
    /// ### Arguments
    ///
    /// * `empty_orders_list` - A list of indices for empty orders. A vector of 2 vectors, where 
    /// the first vector is for asks and the second vector is for bids
    fn clean_up_empty_orders(&mut self, empty_orders_list: Vec<Vec<usize>>) {
        empty_orders_list[0].iter().for_each(|idx| {
            self.asks.remove(*idx);
        });

        empty_orders_list[1].iter().for_each(|idx| {
            self.bids.remove(*idx);
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub address: String,
    pub name: String,
    pub symbol: Option<String>,
    pub total_supply: u64,
    pub highest_bid: Option<String>,
    pub lowest_ask: Option<String>,
}

//------------- TESTS -------------//

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simple_bid(price: f64, quantity: f64) -> Order {
        Order {
            id: String::from("1"),
            listing_id: String::from("1"),
            price,
            quantity,
            is_bid: true,
            created_at: String::from(""),
            druid: None,
            desired_listing_id: None,
        }
    }

    fn create_simple_ask(price: f64, quantity: f64) -> Order {
        Order {
            id: String::from("1"),
            listing_id: String::from("1"),
            price,
            quantity,
            is_bid: false,
            created_at: String::from(""),
            druid: None,
            desired_listing_id: None,
        }
    }

    #[test]
    fn should_add_first_order() {
        //
        // Arrange
        //
        let mut order_book = OrderBook::new();
        let mut order = create_simple_bid(1.0, 1.0);

        //
        // Act
        //
        order_book.add_order(&mut order);

        //
        // Assert
        //
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[0].price, 1.0);
        assert_eq!(order_book.bids[0].id, String::from("1"));
    }

    #[test]
    fn should_match_bid_to_ask() {
        //
        // Arrange
        //
        let mut order_book = OrderBook::new();
        let mut ask = create_simple_ask(1.5, 10.0);
        let mut bid = create_simple_bid(2.0, 3.0);

        // 
        // Act
        //
        order_book.add_order(&mut ask);
        order_book.add_order(&mut bid);

        // 
        // Assert
        //
        assert_eq!(order_book.bids.len(), 0);
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.asks[0].quantity, 7.0);
        assert_eq!(order_book.pending_trades.len(), 1);
        assert_eq!(order_book.pending_trades[0].quantity, 3.0);
        assert_eq!(order_book.pending_trades[0].price, 1.5);
        assert_eq!(order_book.pending_trades[0].bid_id, String::from("1"));
        assert_eq!(order_book.pending_trades[0].ask_id, String::from("1"));
        assert!(order_book.pending_trades[0].druid.len() > 0);
    }

    #[test]
    fn should_match_ask_to_bid() {
        //
        // Arrange
        //
        let mut order_book = OrderBook::new();
        let mut bid = create_simple_bid(1.5, 10.0);
        let mut ask = create_simple_ask(1.0, 3.0);

        //
        // Act
        //
        order_book.add_order(&mut bid);
        order_book.add_order(&mut ask);

        //
        // Assert
        //
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[0].quantity, 7.0);
        assert_eq!(order_book.asks.len(), 0);
        assert_eq!(order_book.pending_trades.len(), 1);
        assert_eq!(order_book.pending_trades[0].quantity, 3.0);
        assert_eq!(order_book.pending_trades[0].price, 1.0);
        assert_eq!(order_book.pending_trades[0].bid_id, String::from("1"));
        assert_eq!(order_book.pending_trades[0].ask_id, String::from("1"));
        assert!(order_book.pending_trades[0].druid.len() > 0);
    }

    #[test]
    fn should_handle_unmatched_orders() {
        //
        // Arrange
        //
        let mut order_book = OrderBook::new();
        let mut bid = create_simple_bid(1.5, 10.0);
        let mut ask = create_simple_ask(2.0, 3.0);

        //
        // Act
        //
        order_book.add_order(&mut bid);
        order_book.add_order(&mut ask);

        //
        // Assert
        //
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[0].quantity, 10.0);
        assert_eq!(order_book.bids[0].id, String::from("1"));
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.asks[0].quantity, 3.0);
        assert_eq!(order_book.asks[0].id, String::from("1"));
        assert_eq!(order_book.pending_trades.len(), 0);
    }
}
