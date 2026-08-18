use std::sync::{Arc, Mutex};
use crate::stock_generator::Stock; 

// Deduct stock from the vector
pub fn deduct_stock(stock_vector: &Arc<Mutex<Vec<Stock>>>, stock_name: &str, quantity: u32) {
    let mut stocks = stock_vector.lock().unwrap();  
    // Find the stock by name
    if let Some(stock) = stocks.iter_mut().find(|s| s.name == stock_name) {  
        if stock.quantity >= quantity {  // Check if there's enough stock to deduct
            stock.quantity -= quantity;  // Deduct the quantity from the stock
            println!("Deducted {} of {}. New quantity: {}", quantity, stock_name, stock.quantity);
        } else {
            println!("Not enough stock of {} to deduct. Current quantity: {}", stock_name, stock.quantity);
        }
    } else {
        println!("Stock {} not found", stock_name);  // If stock not found, print a message
    }
}

// Add quantity to an existing stock in the vector
pub fn add_stock(stock_vector: &Arc<Mutex<Vec<Stock>>>, stock_name: &str, quantity: u32) {
    let mut stocks = stock_vector.lock().unwrap(); 
    // Find the stock by name
    if let Some(stock) = stocks.iter_mut().find(|s| s.name == stock_name) { 
        stock.quantity += quantity; // Add the specified quantity to the stock
        println!(
            "Received {} of {}. Updated quantity: {}",
            quantity, stock_name, stock.quantity
        );
    } else {
        println!(
            "Stock {} not found. Cannot add quantity as the stock doesn't exist.",
            stock_name
        );
    }
}

