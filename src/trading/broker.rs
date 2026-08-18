use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::order::{Order, OrderManager};
use crate::client::{Stock, Client};
use chrono::Local; // Add chrono for timestamp functionality
fn log_with_timestamp(message: &str) {
    let now = Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message);
}
pub struct Broker {
    order_manager: Arc<Mutex<OrderManager>>,
    pub received_messages: Vec<String>,
    pending_orders: Vec<Client>,
    stocks: HashMap<String, Stock>,
}

impl Broker {
    pub fn new(order_manager: Arc<Mutex<OrderManager>>) -> Self {
        Broker {
            order_manager,
            received_messages: Vec::new(),
            pending_orders: Vec::new(),
            stocks: HashMap::new(),
        }
    }


    pub fn receive_orders_from_queue(&mut self) -> Result<Option<Vec<Stock>>, amiquip::Error> {
        let amqp_url = std::env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());
        let mut connection = Connection::insecure_open(&amqp_url)?;
        let channel = connection.open_channel(None)?;
        let queue = channel.queue_declare("stocks", QueueDeclareOptions::default())?;
        let consumer = queue.consume(ConsumerOptions::default())?;
    
        println!("Waiting for messages. Check if receive any stock update.");
    
        loop {
            if consumer.receiver().is_empty() {
                println!("Receiver is empty. Checking one more time...");
    
                std::thread::sleep(std::time::Duration::from_secs(1));
    
                if consumer.receiver().is_empty() {
                    println!("No messages available in the queue.");
                    connection.close()?;
                    return Ok(None);
                }
            }
    
            for message in consumer.receiver().iter() {
                match message {
                    ConsumerMessage::Delivery(delivery) => {
                        let body = String::from_utf8_lossy(&delivery.body).to_string();
                        log_with_timestamp(&format!("Received stock details\n{}", body));
    
                        self.received_messages.push(body.clone());
                        consumer.ack(delivery)?;
    
                        if let Ok(stocks) = self.parse_stock_message(&body) {
                            for stock in &stocks {
                                self.update_stock(stock); 
                            }
    
                            return Ok(Some(stocks));
                        } else {
                            println!("Failed to parse stock message: {}", body);
                        }
                    }
                    other => {
                        println!("Received non-delivery message: {:?}", other);
                        break;
                    }
                }
            }
        }
    }
    
    

    pub fn parse_stock_message(&self, message: &str) -> Result<Vec<Stock>, &'static str> {
        let mut stocks = Vec::new();

        for line in message.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 5 {
                let stock_part = parts[0].split(':').nth(1).unwrap_or("").trim();
                let quantity_part = parts[1].split(':').nth(1).unwrap_or("").trim();
                let price_part = parts[2].split(':').nth(1).unwrap_or("").trim();
                let buy_rate_part = parts[3].split(':').nth(1).unwrap_or("").trim();
                let sell_rate_part = parts[4].split(':').nth(1).unwrap_or("").trim();

                if let (Ok(quantity), Ok(price), Ok(buy_rate), Ok(sell_rate)) = (
                    quantity_part.parse::<u32>(),
                    price_part.trim_start_matches('$').parse::<f64>(),
                    buy_rate_part.trim_start_matches('$').parse::<f64>(),
                    sell_rate_part.trim_start_matches('$').parse::<f64>()
                ) {
                    stocks.push(Stock {
                        name: stock_part.to_string(),
                        quantity,
                        price,
                        buy_rate,
                        sell_rate,
                    });
                } else {
                    return Err("Invalid stock quantity or price format.");
                }
            } else {
                return Err("Invalid message format. Expected 5 parts: Stock, Quantity, Price, Buy Rate, Sell Rate.");
            }
        }

        Ok(stocks)
    }

    pub fn update_stock(&mut self, stock: &Stock) {
        self.stocks
            .entry(stock.name.clone())
            .and_modify(|existing_stock| {
                existing_stock.quantity = stock.quantity;
                existing_stock.price = stock.price;
                existing_stock.buy_rate = stock.buy_rate;
                existing_stock.sell_rate = stock.sell_rate;
            })
            .or_insert_with(|| stock.clone());
    
        println!(
            "Stock updated: {} - Quantity: {}, Price: ${}, Buy Rate: ${}, Sell Rate: ${}",
            stock.name, stock.quantity, stock.price, stock.buy_rate, stock.sell_rate
        );
    }

    pub fn check_order_price(&mut self, client: &Client, stock: &Stock, action: &str) {
        // Calculate the total cost of the order (price per stock * quantity)
        let total_buy_cost = stock.buy_rate * stock.quantity as f64;
        let total_sell_cost = stock.sell_rate * stock.quantity as f64;

        match action {
                "Buy" => {
                if client.max_price >= total_buy_cost {
                    println!(
                        "Order Successful: Client {} can buy {} {} at ${} each. Total buy cost: ${}.",
                        client.id, stock.quantity, stock.name, stock.buy_rate, total_buy_cost
                    );
                    self.place_order(client, stock, action);
                } else {
                    println!(
                        "Order Failed: Client {} cannot buy {} {} at ${} each. Total buy cost: ${}. Max price: ${}.",
                        client.id, stock.quantity, stock.name, stock.buy_rate, total_buy_cost, client.max_price
                    );
                    self.add_to_pending_orders(client, stock, action); // Only add the failed stock
                }
            }
            "Sell" => {
                if client.max_price >= total_sell_cost {
                    println!(
                        "Order Successful: Client {} can sell {} {} at ${} each. Total sell cost: ${}.",
                        client.id, stock.quantity, stock.name, stock.sell_rate, total_sell_cost
                    );
                    self.place_order(client, stock, action);
                } else {
                    println!(
                        "Order Failed: Client {} cannot sell {} {} at ${} each. Total sell cost: ${}. Max price: ${}.",
                        client.id, stock.quantity, stock.name, stock.sell_rate, total_sell_cost, client.max_price
                    );
                    self.add_to_pending_orders(client, stock, action); // Only add the failed stock
                }
            }
            _ => {
                println!("Invalid action: {}", action);
            }
        }
    }
    

    pub fn add_to_pending_orders(&mut self, client: &Client, stock: &Stock, action: &str) {
        // Clone the client and modify their stocks to include only the failed stock
        let mut client_with_action = client.clone();
        
        // Create a new list of stocks with only the failed stock
        let failed_stock = Stock {
            name: stock.name.clone(),
            quantity: stock.quantity,
            price: stock.price,
            buy_rate: stock.buy_rate,
            sell_rate: stock.sell_rate,
        };
        
        client_with_action.stocks.clear(); // Clear all previous stocks
        client_with_action.stocks.push(failed_stock); // Add the failed stock
        
        // Update the action field
        client_with_action.action = action.to_string();
    
        // Add the client with the action and failed stock to the pending orders list
        self.pending_orders.push(client_with_action);
        
        // Print confirmation message
        println!(
            "Client {} with action {} and failed stock {} added to pending orders. Waiting for price to match.",
            client.id, action, stock.name
        );
    }
    


    pub fn process_pending_orders(&mut self) {
        if self.pending_orders.is_empty() {
            return;
        }
    
        println!("Pending orders list:");
        for client in &self.pending_orders {
            println!("Client ID: {}, Max Price: ${}", client.id, client.max_price);
            for stock in &client.stocks {
                println!("Stock: {}, Quantity: {}, Price: ${}, Buy Rate: ${}, Sell Rate: ${}\n", stock.name, stock.quantity, stock.price, stock.buy_rate, stock.sell_rate);
            }
        }
    
        let mut i = 0;
        while i < self.pending_orders.len() {
            let client = self.pending_orders[i].clone();  
    
            println!("Processing pending order for Client {}: Max Price ${}", client.id, client.max_price);
    

            match client.action.as_str() {
                "Buy" => {
                    println!("Client {} is a buy order.",client.id);
                    if self.process_buy_order(&client, i) {
                        // If the buy order is processed, remove the client from the pending orders list and move to the next client
                        self.pending_orders.remove(i);
                        continue;
                    }
                }
                "Sell" => {
                    println!("Client {} is a sell order.", client.id);
                    if self.process_sell_order(&client, i) {
                        // If the sell order is processed, remove the client from the pending orders list and move to the next client
                        self.pending_orders.remove(i);
                        continue;
                    }
                }
                _ => {
                    println!("Unknown action for Client {}: {}", client.id, client.action);
                    i += 1; // Skip if action is unknown
                }
            }
            i += 1; // Proceed to next client if the current client was not processed
        }
    }
    
    fn process_buy_order(&mut self, client: &Client, i: usize) -> bool {
        let mut order_processed = false;
    
        for stock in &client.stocks {
            if let Some(current_stock) = self.stocks.get_mut(&stock.name) {
                // Use the buy rate for total cost calculation
                let total_buy_cost = stock.quantity as f64 * current_stock.buy_rate;
    
                println!(
                    "Client stock {}: Client {}, Quantity {}, Max Price ${}",
                    stock.name, client.id, stock.quantity, client.max_price
                );
                println!(
                    "Current stock {} buy rate {} and total cost ${}\n",
                    current_stock.name,current_stock.buy_rate, total_buy_cost
                );
    
                if client.max_price >= total_buy_cost && current_stock.quantity >= stock.quantity {
                    println!(
                        "Buy Order Successful: Client {} can now buy {} {} at ${}.",
                        client.id, stock.quantity, stock.name, total_buy_cost
                    );
    
                    // Update the stock quantity after successful buy
                    current_stock.quantity -= stock.quantity;
    
                    // Place the order
                    self.place_order(client, stock, &client.action);
    
                    order_processed = true;
                    break;
                } else if current_stock.quantity < stock.quantity {
                    println!(
                        "Insufficient stock: {}. Available: {}, Requested: {}",
                        stock.name, current_stock.quantity, stock.quantity
                    );
                    return false; // Insufficient stock
                } else {
                    println!("Client {} failed to proceed with buy.", client.id);
                    return false; // Return false to indicate failure
                }
            } else {
                println!("Stock {} not found in current stock list.", stock.name);
                return false; // Return false to indicate failure
            }
        }
    
        if order_processed {
            println!("Buy Order processed and removed for Client {}", client.id);
            return true; // Indicate that the order was successfully processed
        }
    
        false // Return false if no order was processed
    }
    
    fn process_sell_order(&mut self, client: &Client, i: usize) -> bool {
        let mut order_processed = false;
    
        for stock in &client.stocks {
            if let Some(current_stock) = self.stocks.get_mut(&stock.name) {
                // Use the sell rate for total cost calculation
                let total_sell_cost = stock.quantity as f64 * current_stock.sell_rate;
    
                println!(
                    "Client stock {}: Client {}, Quantity {}, Max Price ${}",
                    stock.name, client.id, stock.quantity, client.max_price
                );
                println!(
                    "Current stock {} sell rate {} and total cost ${}\n",
                    current_stock.name, current_stock.sell_rate, total_sell_cost
                );
    
                if client.max_price >= total_sell_cost && current_stock.quantity >= stock.quantity {
                    println!(
                        "Sell Order Successful: Client {} can now sell {} {} at ${}.",
                        client.id, stock.quantity, stock.name, total_sell_cost
                    );
    
                    // Update the stock quantity after successful sell
                    current_stock.quantity += stock.quantity;
    
                    println!(
                        "Stock {} updated after sell. New quantity: {}.",
                        stock.name, current_stock.quantity
                    );
    
                    // Place the order
                    self.place_order(client, stock, &client.action);
    
                    order_processed = true;
                    break;
                } else if current_stock.quantity < stock.quantity {
                    println!(
                        "Insufficient stock: {}. Available: {}, Requested: {}",
                        stock.name, current_stock.quantity, stock.quantity
                    );
                    return false; // Insufficient stock
                } else {
                    println!("Client {} failed to proceed with sell.", client.id);
                    return false; // Return false to indicate failure
                }
            } else {
                println!("Stock {} not found in current stock list.", stock.name);
                return false; // Return false to indicate failure
            }
        }
    
        if order_processed {
            println!("Sell Order processed and removed for Client {}", client.id);
            return true; // Indicate that the order was successfully processed
        }
    
        false // Return false if no order was processed
    }
    

    

pub fn place_order(&mut self, client: &Client, stock: &Stock, action: &str) {
    if let Some(current_stock) = self.stocks.get_mut(&stock.name) {
        match action {
            "Buy" => {
                if current_stock.quantity >= stock.quantity {
                    // Reduce stock quantity for a successful buy order
                    current_stock.quantity -= stock.quantity;
                    println!(
                        "Buy Order Successful: Stock {} reduced by {}. Remaining quantity: {}.",
                        stock.name, stock.quantity, current_stock.quantity
                    );
                } else {
                    println!(
                        "Buy Order Failed: Not enough stock of {}. Available: {}, Requested: {}.",
                        stock.name, current_stock.quantity, stock.quantity
                    );
                    self.add_to_pending_orders(client, stock, action);
                    return;
                }
            }
            "Sell" => {
                // Increase stock quantity for a successful sell order
                current_stock.quantity += stock.quantity;
                println!(
                    "Sell Order Successful: Stock {} increased by {}. New quantity: {}.",
                    stock.name, stock.quantity, current_stock.quantity
                );
            }
            _ => {
                println!("Invalid action: {}", action);
                return;
            }
        }
    } else {
        println!("Stock {} not found. Unable to process the order.", stock.name);
        return;
    }

    // Create an order record and add it to the order manager
    let mut stock_map = HashMap::new();
    stock_map.insert(stock.name.clone(), stock.quantity);

    let order = Order::new(client.id, action.to_string(), "Successful".to_string(), stock_map);
    let mut order_manager = self.order_manager.lock().unwrap();
    order_manager.add_order(order);

    println!(
        "Order placed for Client {}: {} {} of {}.\n",
        client.id, action, stock.quantity, stock.name
    );
}

    
}
