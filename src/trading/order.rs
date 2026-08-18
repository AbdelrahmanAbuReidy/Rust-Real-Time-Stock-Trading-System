use amiquip::{Connection, Exchange, Publish, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an order placed by a client.
#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub client_id: u32,
    pub action: String, // "Buy" or "Sell"
    pub status: String, // "Pending", "Successful", etc.
    pub stocks: HashMap<String, u32>, // Stocks that are part of the order (name -> quantity)
}

impl Order {
    /// Creates a new order with initial values.
    pub fn new(client_id: u32, action: String, status: String, stocks: HashMap<String, u32>) -> Self {
        Order {
            client_id,
            action,
            status,
            stocks,
        }
    }

    /// Returns a formatted string of the stocks in the order for easier logging.
    pub fn format_stocks(&self) -> String {
        self.stocks.iter()
            .map(|(name, quantity)| format!("{}: {}", name, quantity))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// Struct to represent the stock info for publishing to RabbitMQ
#[derive(Serialize)]
struct StockInfo {
    name: String,
    quantity: u32,
    action: String,
}

pub struct OrderManager {
    orders: Vec<Order>,
}

impl OrderManager {
    pub fn new() -> Self {
        OrderManager {
            orders: Vec::new(),
        }
    }


        pub fn add_order(&mut self, order: Order) {
            // Display the new order
            let stock_details = order.format_stocks();
            println!(
                "New order created: Client ID: {}, Action: {}, Status: {}, Stocks: [{}]",
                order.client_id, order.action, order.status, stock_details
            );
    
            // Send the order to RabbitMQ if it's successful
            if order.status == "Successful" {
                if let Err(e) = self.send_order_to_queue(&order) {
                    eprintln!("Failed to send order to RabbitMQ: {}", e);
                }
            }
    

            self.orders.push(order);
        }

    /// Sends the order to RabbitMQ with only stock name, quantity, and action.
    fn send_order_to_queue(&self, order: &Order) -> Result<()> {
        for (stock_name, quantity) in &order.stocks {
            // Create the StockInfo struct with correct types.
            let stock_info = StockInfo {
                name: stock_name.clone(),
                quantity: *quantity, // quantity is passed as u32
                action: order.action.clone(),
            };

            // Serialize the StockInfo struct to JSON.
            let stock_info_json = serde_json::to_string(&stock_info).unwrap();

            // Establish connection to RabbitMQ.
            let amqp_url = std::env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());
            let mut connection = Connection::insecure_open(&amqp_url)?;
            let channel = connection.open_channel(None)?;

            // Get a handle to the direct exchange on the channel.
            let exchange = Exchange::direct(&channel);

            // Publish the stock info JSON string to the "order_queue".
            exchange.publish(Publish::new(stock_info_json.as_bytes(), "order_queue"))?;

            // Close the connection after publishing the message.
            connection.close()?;
        }

        Ok(())
    }


    /// Prints all orders in the system with their details.
    pub fn print_orders(&self) {
        for order in &self.orders {
            let stock_details = order.format_stocks();
            println!("All created order");
            println!(
                "Client ID: {}, Action: {}, Status: {}, Stocks: [{}]",
                order.client_id, order.action, order.status, stock_details
            );
        }
    }
}
