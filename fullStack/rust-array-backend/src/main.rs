use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, get, post, delete};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct Order {
    price: f64,
    quantity: f64,
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
    order_book.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    "Bid added"
}

#[post("/add_ask")]
async fn add_ask(order: web::Json<Order>, order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let mut order_book = order_book.lock().unwrap();
    order_book.asks.push(order.into_inner());
    order_book.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    "Ask added"
}

#[derive(Deserialize)]
struct PriceQuery {
    price: f64,
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
