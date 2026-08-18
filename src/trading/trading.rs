mod client;
mod broker;
mod order;

use broker::Broker;
use order::OrderManager;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use client::{generate_client, Client, Stock};
use chrono::Local; // Add chrono for timestamp functionality
use cpu_monitor::{CpuInstant};


// Helper function to print messages with a timestamp
fn log_with_timestamp(message: &str) {
    let now = Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message);
}

pub fn cpu_mon() -> Result<(), std::io::Error> {
    thread::sleep(Duration::from_secs(1)); 

    let start = CpuInstant::now()?;
    loop {
        let end = CpuInstant::now()?;
        let duration = end - start;
        
        // Avoid printing NaN during the first iteration
        if duration.non_idle().is_finite() {
            println!("CPU Usage: {:.0}%\n", duration.non_idle() * 100.);
        }
        
        std::thread::sleep(Duration::from_secs(1)); 
    }
}

fn main() {
    let min_clients = 1;
    let max_clients = 5;
    let client_gen_interval = Duration::from_secs(1);
    let runtime = Duration::from_secs(90);
    let start_time = Instant::now();
    let mut client_id = 1; 

    let order_manager = Arc::new(Mutex::new(OrderManager::new()));
    let broker = Arc::new(Mutex::new(Broker::new(order_manager.clone())));

    let broker_clone = Arc::clone(&broker);
    log_with_timestamp("Stock Trading System start....\n");


    let broker_locked = Arc::clone(&broker);
    thread::spawn(move || {
        let message_check_interval = Duration::from_secs(1);
        let start_time = Instant::now();
    
        while Instant::now().duration_since(start_time) < runtime {
            if let Ok(mut broker_locked) = broker_locked.lock() {
                if let Ok(Some(stocks)) = broker_locked.receive_orders_from_queue() {
                    println!("Stocks updated from queue.\n");
                } else {
                }
            } else {
                println!("Failed to acquire lock on broker.");
            }

            thread::sleep(message_check_interval);
        }
    });

    thread::spawn(move || {
        let process_interval = Duration::from_secs(3);  // Process pending orders every 3 seconds

        while Instant::now().duration_since(start_time) < runtime {
            if let Ok(mut broker_locked) = broker_clone.lock() {
                broker_locked.process_pending_orders();  // Re-check and process pending orders
            }
            thread::sleep(process_interval);
        }
    });

    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            if let Err(e) = cpu_mon() {
                eprintln!("Error in CPU monitoring: {}", e);
            }
            thread::sleep(Duration::from_secs(1)); 
        }
    });

    let mut rng = thread_rng();
    while Instant::now().duration_since(start_time) < runtime {
        let num_clients = rng.gen_range(min_clients..=max_clients);

        for _ in 0..num_clients {

            let mut broker_locked = broker.lock().unwrap();
            if !broker_locked.received_messages.is_empty() {
                let received_msgs = broker_locked.received_messages.clone(); 
               
                for msg in received_msgs {
                    match broker_locked.parse_stock_message(&msg) {
                        Ok(stocks) => {
                            if let Some(client) = generate_client(&mut client_id, &stocks) {
                                log_with_timestamp(&format!("\nGenerated Client ID: {}", client.id));
                                println!("Action: {}", client.action);
                                for (i, stock) in client.stocks.iter().enumerate() {
                                    println!(
                                        "({:>3}) Stock: {} - Quantity: {}, Max Price: ${}",
                                        i + 1, 
                                        stock.name,
                                        stock.quantity,
                                        client.max_price
                                    );
                                    println!(
                                        "Receive Client Details, Send to Broker to Proceed The Order\n"
                                    );
                                    broker_locked.check_order_price(&client, stock, &client.action);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to parse message: {}. Error: {}", msg, e);
                        }
                    }
                }
            }
        }
        thread::sleep(client_gen_interval);
    }


    log_with_timestamp("\nProcessing completed.");

    println!("\nSimulation complete after 90 seconds.");
    let order_manager_locked = order_manager.lock().unwrap();
    order_manager_locked.print_orders();
    
}
