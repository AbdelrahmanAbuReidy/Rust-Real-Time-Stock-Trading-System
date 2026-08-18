use rand::Rng;
use std::sync::{Arc, Mutex};

use crate::stock_generator::Stock;  

// Function to simulate price fluctuation for a given stock
pub fn fluctuate_price(stock: &mut Stock) {
    let mut rng = rand::thread_rng();
    let fluctuation: f64 = rng.gen_range(-0.05..0.05); // Random fluctuation between -5% and 5%
    let percentage_change = fluctuation * 100.0; // Convert fluctuation to percentage
    let price_change = stock.price * fluctuation;

    // Print the fluctuation rate
    if fluctuation > 0.0 {
        println!(
            "Stock price for {} will increase by {:+.2}%",
            stock.name, percentage_change
        );
    } else {
        println!(
            "Stock price for {} will decrease by {:+.2}%",
            stock.name, percentage_change
        );
    }

    // Update the stock price
    stock.price += price_change;

    // Ensure price doesn't go negative
    if stock.price < 0.0 {
        stock.price = 0.0;
    }

    // Update the buy rate and sell rate
    stock.buy_rate = stock.price * 0.99;  // Buy rate is 1% lower than the new price
    stock.sell_rate = stock.price * 1.01; // Sell rate is 1% higher than the new price
}


pub fn update_stock_prices(stock_vector: Arc<Mutex<Vec<Stock>>>) {
    // Lock the stock_vector inside this function
    let mut stocks = stock_vector.lock().unwrap();  

    //update the stock prices
    for stock in stocks.iter_mut() {
        fluctuate_price(stock); // Update each stock's price
    }
}
