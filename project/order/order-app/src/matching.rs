use std::io;

use serde::{Deserialize};
use reqwest::Error;

#[derive(Debug, Clone)]

struct Order {
    price: f64,
    quantity: f64,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

async fn fetch_data() -> Result<ApiResponse, Error> {
    let response = reqwest::get("http://127.0.0.1:8080/array")
        .await?
        .json::<ApiResponse>()
        .await?;
    
    Ok(response)
}

fn get_orderbook {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn match_orders(buy_orders: &mut Vec<Order>, ask_orders: &mut Vec<Order>) -> Vec<(Order, Order)> {
    // Sort buy orders by price in descending order
    buy_orders.sort_by(|a, b| b.price.cmp(&a.price));
    
    // Sort ask orders by price in ascending order
    ask_orders.sort_by(|a, b| a.price.cmp(&b.price));
    
    let mut matched_orders = Vec::new();
    
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
            matched_orders.push((
                Order { price: buy_order.price, quantity: matched_quantity, address: buy_order.address.clone() },
                Order { price: ask_order.price, quantity: matched_quantity, address: ask_order.address.clone() }
            ));
            
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
    
    matched_orders
}


async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data().await?;
    println!("{:?}", data);
    Ok(())
}

fn main() {
    let mut buy_orders = vec![
        Order { price: 105, quantity: 10 , address: "0x123".to_string()},
        Order { price: 100, quantity: 5 , address: "0x123".to_string()},
        Order { price: 90, quantity: 5 , address: "0x123".to_string()},
    ];
    
    let mut ask_orders = vec![
        Order { price: 99, quantity: 8, address: "0x123".to_string()},
        Order { price: 98, quantity: 15, address: "0x123".to_string()},
    ];
    
    let matched = match_orders(&mut buy_orders, &mut ask_orders);
    println!("{:?}", matched);
}
