use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, get, post, delete};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::fs;
use std::io::{self, Write};

use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    PredicateAddress, Value, Word, ContentAddress,
};


#[derive(Serialize, Deserialize, Clone)]
struct Order {
    price: i64,
    quantity: i64,
    addresses: Vec<PredicateAddress>,
}

#[derive(Serialize, Clone)]
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
}



#[get("/array")]
async fn get_array(order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let order_book = order_book.lock().unwrap();
    web::Json(order_book.clone())
}

#[post("/add_bid")]
async fn add_bid(order: web::Json<Order>, order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    order_book.bids.push(order.into_inner());
    order_book.bids.sort_by(|a, b| b.price.cmp(&a.price));
    "Bid added"
}

#[post("/add_ask")]
async fn add_ask(order: web::Json<Order>, order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    order_book.asks.push(order.into_inner());
    order_book.asks.sort_by(|a, b| a.price.cmp(&b.price));
    "Ask added"
}

#[derive(Deserialize)]
struct PriceQuery {
    price: i64,
}

#[delete("/remove_bid")]
async fn remove_bid(query: web::Query<PriceQuery>, order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    order_book.bids.retain(|order| order.price != query.price);
    "Bid removed"
}

#[delete("/remove_ask")]
async fn remove_ask(query: web::Query<PriceQuery>, order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    order_book.asks.retain(|order| order.price != query.price);
    "Ask removed"
}

fn match_orders(buy_orders: &mut Vec<Order>, ask_orders: &mut Vec<Order>) -> (Vec<Order>, Vec<Order>, Vec<Order>, Vec<Order>) {
    // Sort buy orders by price in descending order
    buy_orders.sort_by(|a, b| b.price.cmp(&a.price));
    
    // Sort ask orders by price in ascending order
    ask_orders.sort_by(|a, b| a.price.cmp(&b.price));
    
    let mut matched_orders_bids = Vec::new();
    let mut matched_orders_asks = Vec::new();

    while !buy_orders.is_empty() && !ask_orders.is_empty() {
        let mut buy_order = buy_orders[0].clone();
        let mut ask_order = ask_orders[0].clone();
        
        // If the highest buy order is greater than or equal to the lowest ask order, match them
        if buy_order.price >= ask_order.price {
            let matched_quantity = buy_order.quantity.min(ask_order.quantity);
            
            // Reduce the quantities of both buy and ask orders
            buy_order.quantity -= matched_quantity;
            ask_order.quantity -= matched_quantity;
            
            // Save the matched order
            matched_orders_bids.push(
                Order { price: buy_order.price, quantity: matched_quantity, addresses: buy_order.addresses.clone() }
            );
            matched_orders_asks.push(
                Order { price: ask_order.price, quantity: matched_quantity, addresses: ask_order.addresses.clone() }
            );
            
            // Remove buy or ask orders if they are fully filled
            if buy_order.quantity == 0 {
                buy_orders.remove(0);
            } else {
                buy_orders[0].quantity = buy_order.quantity;
            }
            
            if ask_order.quantity == 0 {
                ask_orders.remove(0);
            } else {
                ask_orders[0].quantity = ask_order.quantity;
            }
        } else {
            // No more profitable trades can be made
            break;
        }
    }
    (matched_orders_bids, matched_orders_asks, buy_orders.clone(), ask_orders.clone())
}

/*
//EXAMPLE
const BID_AMOUNT_KEY: Word = 0;
const PRICE_KEY: Word = 1;
const OWNER_KEY: Word = 2;
const NONCE_KEY: Word = 3;

#[derive(Clone)]
pub struct QueryVars {
    pub bid_amount: Option<Value>,
    pub price: Option<Value>,
    pub owner: Option<Value>,
    pub nonce: Option<Value>,
}

#[derive(Clone)]
pub struct StateKey(pub Vec<Word>);

pub fn bid_amount_key() -> StateKey {
    StateKey(vec![BID_AMOUNT_KEY])
}

pub fn price_key() -> StateKey {
    StateKey(vec![PRICE_KEY])
}

pub fn owner_key() -> StateKey {
    StateKey(vec![OWNER_KEY])
}

pub fn nonce_key() -> StateKey {
    StateKey(vec![NONCE_KEY])
}

/// Extract the bid amount from the query result.
pub fn extract_bid_amount(query: QueryVars) -> anyhow::Result<Word> {
    match query.bid_amount {
        Some(bid_amount) => match &bid_amount[..] {
            [] => Ok(0),
            [bid_amount] => Ok(*bid_amount),
            _ => bail!("Expected single word, got: {:?}", bid_amount),
        },
        None => Ok(0),
    }
}

/// Extract the price from the query result.
pub fn extract_price(query: QueryVars) -> anyhow::Result<Word> {
    match query.price {
        Some(price) => match &price[..] {
            [] => Ok(0),
            [price] => Ok(*price),
            _ => bail!("Expected single word, got: {:?}", price),
        },
        None => Ok(0),
    }
}

/// Extract the owner from the query result.
pub fn extract_owner(query: QueryVars) -> anyhow::Result<Word> {
    match query.owner {
        Some(owner) => match &owner[..] {
            [] => Ok(0),
            [owner] => Ok(*owner),
            _ => bail!("Expected single word, got: {:?}", owner),
        },
        None => Ok(0),
    }
}

/// Extract the nonce from the query result.
pub fn extract_nonce(query: QueryVars) -> anyhow::Result<Word> {
    match query.nonce {
        Some(nonce) => match &nonce[..] {
            [] => Ok(0),
            [nonce] => Ok(*nonce),
            _ => bail!("Expected single word, got: {:?}", nonce),
        },
        None => Ok(0),
    }
}

/// Create a solution to place a bid.
pub fn create_bid_solution(
    predicate: PredicateAddress,
    amount: Word,
) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: amount,
            transient_data: Default::default(),
            state_mutations: vec![
                Mutation {
                    key: vec![BID_AMOUNT_KEY],
                    value: vec![amount],
                },
            ],
        }],
    }
}

/// Create a solution to place an ask.
pub fn create_ask_solution(
    predicate: PredicateAddress,
    amount: Word,
) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![
                Mutation {
                    key: vec![BID_AMOUNT_KEY],
                    value: vec![amount],
                },
                Mutation {
                    key: vec![PRICE_KEY],
                    value: vec![0],
                },
            ],
        }],
    }
}

/// Create a solution to update the bid amount and price.
pub fn create_update_solution(
    predicate: PredicateAddress,
    new_bid_amount: Word,
    new_price: Word,
) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![
                Mutation {
                    key: vec![BID_AMOUNT_KEY],
                    value: vec![new_bid_amount],
                },
                Mutation {
                    key: vec![PRICE_KEY],
                    value: vec![new_price],
                },
            ],
        }],
    }
}
*/

#[post("/update")]
async fn match_update(order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    let mut buy_orders = order_book.bids.clone();
    let mut ask_orders = order_book.asks.clone();
    let (matched_orders_bids, matched_orders_asks, new_buys, new_asks) = match_orders(&mut buy_orders, &mut ask_orders);

    /*
    let a = 0; //"amount"
    let b = 0; //"amount * price"

    
    for matched_order in matched_orders_bids {
        let amount = extract_bid_amount(matched_order.addresses[0].query_vars.clone()).unwrap();
        let price = extract_price(matched_order.addresses[0].query_vars.clone()).unwrap();

        let new_amount = amount - matched_order.quantity;
        
        a = a + matched_order.quantity;
        b = b - matched_order.quantity * price;

        //do token transfer

        let new_solution = create_bid_solution(matched_order.addresses[0].clone(), new_amount);

        //create and send solution

    }

    for matched_order in matched_orders_asks {

    }
    */

    order_book.bids = buy_orders;
    order_book.asks = ask_orders;

    order_book.bids.sort_by(|a, b| b.price.cmp(&a.price));

    order_book.asks.sort_by(|a, b| a.price.cmp(&b.price));


    "updated"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let order_book = Arc::new(Mutex::new(OrderBook {
        bids: vec![],
        asks: vec![],
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(order_book.clone()))
            .wrap(Cors::permissive())
            .service(get_array)
            .service(add_bid)
            .service(add_ask)
            .service(remove_bid)
            .service(remove_ask)
            .service(match_update)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
