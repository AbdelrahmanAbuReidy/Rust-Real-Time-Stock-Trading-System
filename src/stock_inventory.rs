use crate::order_management::{deduct_stock, add_stock}; 
use crate::stock_generator::{Stock, StockTracker}; 

use amiquip::{Channel, ConsumerMessage, ConsumerOptions,QueueDeclareOptions};
use std::sync::{Arc, Mutex};
use serde_json;

pub fn receive_stocks(channel: Arc<Mutex<Channel>>, stock_vector: Arc<Mutex<Vec<Stock>>>,tracker: Arc<Mutex<StockTracker>>,) {
    let channel = channel.lock().unwrap();
    let queue = channel.queue_declare("order_queue", QueueDeclareOptions::default()).unwrap();
    let consumer: amiquip::Consumer<'_> = queue.consume(ConsumerOptions::default()).unwrap();

    println!("Waiting for stocks...");

    for (i,message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                match serde_json::from_str::<Order>(&body) {
                    Ok(order) => {
                        match order.action {
                            Action::Buy => {
                                println!("Buying {} of {}", order.quantity, order.name);
                                // Call function to deduct stock
                                deduct_stock(&stock_vector, &order.name, order.quantity);

                                // Record the buy transaction
                                let mut tracker = tracker.lock().unwrap();
                                tracker.record_transaction(&order.name, order.quantity, "bought");
                            }
                            Action::Sell => {
                                println!("Selling {} of {}", order.quantity, order.name);
                                // Call function to add stock
                                add_stock(&stock_vector, &order.name, order.quantity);

                                 // Record the sell transaction
                                 let mut tracker = tracker.lock().unwrap();
                                 tracker.record_transaction(&order.name, order.quantity, "sold");
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to deserialize message: {}", e);
                        // Log the raw message for debugging
                        println!("Raw message: {}", body);
                    }
                }
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Order {
    pub name: String,
    pub quantity: u32,
    pub action: Action,
}

#[derive(Debug, serde::Deserialize)]
pub enum Action {
    Buy,
    Sell,
}
