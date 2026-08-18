pub mod stock_generator;
pub mod order_management;        
pub mod stock_price;         
pub mod stock_inventory; 

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Define the Stock struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock {
    pub name: String,
    pub quantity: u32,
    pub price: f64,
    pub buy_rate: f64,
    pub sell_rate: f64,
}

pub fn generate_stock_vector() -> Arc<Mutex<Vec<Stock>>> {
    let stocks = vec![
        Stock { name: "MSFT".to_string(), quantity: 359, price: 362.88, buy_rate: 359.25, sell_rate: 366.51 },
        Stock { name: "AAPL".to_string(), quantity: 135, price: 218.11, buy_rate: 215.93, sell_rate: 220.29 },
        Stock { name: "IBM".to_string(), quantity: 189, price: 479.29, buy_rate: 474.50, sell_rate: 484.08 },
        Stock { name: "DELL".to_string(), quantity: 185, price: 307.35, buy_rate: 304.28, sell_rate: 310.42 },
        Stock { name: "GOOG".to_string(), quantity: 268, price: 333.36, buy_rate: 330.03, sell_rate: 336.69 },
        Stock { name: "AMZN".to_string(), quantity: 368, price: 446.39, buy_rate: 441.93, sell_rate: 450.85 },
        Stock { name: "TSLA".to_string(), quantity: 263, price: 142.46, buy_rate: 141.04, sell_rate: 143.88 },
        Stock { name: "NVDA".to_string(), quantity: 384, price: 353.67, buy_rate: 350.14, sell_rate: 357.20 },
        Stock { name: "META".to_string(), quantity: 86, price: 95.36, buy_rate: 94.41, sell_rate: 96.31 },
        Stock { name: "ORCL".to_string(), quantity: 87, price: 305.12, buy_rate: 302.07, sell_rate: 308.17 },
        Stock { name: "ADBE".to_string(), quantity: 242, price: 88.13, buy_rate: 87.25, sell_rate: 89.01 },
        Stock { name: "NFLX".to_string(), quantity: 441, price: 350.93, buy_rate: 347.42, sell_rate: 354.44 },
        Stock { name: "CSCO".to_string(), quantity: 104, price: 61.84, buy_rate: 61.22, sell_rate: 62.46 },
        Stock { name: "INTC".to_string(), quantity: 318, price: 219.83, buy_rate: 217.63, sell_rate: 222.03 },
        Stock { name: "AMD".to_string(), quantity: 344, price: 299.38, buy_rate: 296.39, sell_rate: 302.37 },
        Stock { name: "SAP".to_string(), quantity: 152, price: 456.27, buy_rate: 451.71, sell_rate: 460.83 },
        Stock { name: "CRM".to_string(), quantity: 407, price: 424.51, buy_rate: 420.26, sell_rate: 428.76 },
        Stock { name: "QCOM".to_string(), quantity: 307, price: 320.81, buy_rate: 317.60, sell_rate: 324.02 },
        Stock { name: "TXN".to_string(), quantity: 379, price: 393.63, buy_rate: 389.69, sell_rate: 397.56 },
        Stock { name: "AVGO".to_string(), quantity: 452, price: 71.04, buy_rate: 70.33, sell_rate: 71.75 },
        Stock { name: "BABA".to_string(), quantity: 254, price: 349.52, buy_rate: 346.02, sell_rate: 353.02 },
        Stock { name: "PYPL".to_string(), quantity: 100, price: 202.16, buy_rate: 200.14, sell_rate: 204.18 },
        Stock { name: "INTU".to_string(), quantity: 292, price: 363.26, buy_rate: 359.63, sell_rate: 366.89 },
        Stock { name: "SNOW".to_string(), quantity: 41, price: 470.55, buy_rate: 465.84, sell_rate: 475.25 },
        Stock { name: "ZM".to_string(), quantity: 216, price: 207.11, buy_rate: 205.04, sell_rate: 209.18 },
        Stock { name: "SHOP".to_string(), quantity: 471, price: 202.47, buy_rate: 200.45, sell_rate: 204.49 },
        Stock { name: "UBER".to_string(), quantity: 202, price: 406.07, buy_rate: 402.01, sell_rate: 410.13 },
        Stock { name: "SPOT".to_string(), quantity: 17, price: 371.04, buy_rate: 367.33, sell_rate: 374.75 },
        Stock { name: "SQ".to_string(), quantity: 475, price: 185.48, buy_rate: 183.63, sell_rate: 187.33 },
        Stock { name: "TWLO".to_string(), quantity: 292, price: 216.77, buy_rate: 214.60, sell_rate: 218.94 },
        Stock { name: "DOCU".to_string(), quantity: 95, price: 120.09, buy_rate: 118.89, sell_rate: 121.29 },
        Stock { name: "OKTA".to_string(), quantity: 331, price: 444.65, buy_rate: 440.20, sell_rate: 449.10 },
        Stock { name: "CRWD".to_string(), quantity: 175, price: 221.5, buy_rate: 219.29, sell_rate: 223.71 },
        Stock { name: "PLTR".to_string(), quantity: 263, price: 214.13, buy_rate: 211.99, sell_rate: 216.27 },
        Stock { name: "FSLY".to_string(), quantity: 422, price: 475.7, buy_rate: 470.94, sell_rate: 480.46 },
        Stock { name: "MDB".to_string(), quantity: 463, price: 128.25, buy_rate: 126.97, sell_rate: 129.53 },
        Stock { name: "TTD".to_string(), quantity: 385, price: 200.95, buy_rate: 198.94, sell_rate: 202.96 },
        Stock { name: "RBLX".to_string(), quantity: 420, price: 256.84, buy_rate: 254.27, sell_rate: 259.41 },
        Stock { name: "EA".to_string(), quantity: 59, price: 235.34, buy_rate: 233.00, sell_rate: 237.68 },
        Stock { name: "ATVI".to_string(), quantity: 300, price: 132.52, buy_rate: 131.19, sell_rate: 133.85 },
        Stock { name: "ROKU".to_string(), quantity: 483, price: 143.03, buy_rate: 141.60, sell_rate: 144.46 },
        Stock { name: "NET".to_string(), quantity: 29, price: 339.98, buy_rate: 336.58, sell_rate: 343.38 },
        Stock { name: "ASML".to_string(), quantity: 426, price: 178.66, buy_rate: 176.87, sell_rate: 180.45 },
        Stock { name: "VMW".to_string(), quantity: 54, price: 193.79, buy_rate: 191.85, sell_rate: 195.73 },
        Stock { name: "WDAY".to_string(), quantity: 349, price: 490.16, buy_rate: 485.26, sell_rate: 495.06 },
        Stock { name: "NOW".to_string(), quantity: 269, price: 182.91, buy_rate: 181.08, sell_rate: 184.74 },
        Stock { name: "Z".to_string(), quantity: 36, price: 303.69, buy_rate: 300.65, sell_rate: 306.73 },
        Stock { name: "WORK".to_string(), quantity: 91, price: 294.78, buy_rate: 291.83, sell_rate: 297.73 },
        Stock { name: "DDOG".to_string(), quantity: 189, price: 293.87, buy_rate: 290.93, sell_rate: 296.81 },
        Stock { name: "SPLK".to_string(), quantity: 289, price: 126.58, buy_rate: 125.31, sell_rate: 127.85 },
        Stock { name: "PINS".to_string(), quantity: 196, price: 307.83, buy_rate: 304.75, sell_rate: 310.91 },
        Stock { name: "ETSY".to_string(), quantity: 143, price: 188.87, buy_rate: 187.00, sell_rate: 190.74 },
        Stock { name: "TEAM".to_string(), quantity: 264, price: 101.03, buy_rate: 100.02, sell_rate: 102.04 },
        Stock { name: "ALGN".to_string(), quantity: 426, price: 248.07, buy_rate: 245.59, sell_rate: 250.55 },
        Stock { name: "PTON".to_string(), quantity: 218, price: 212.51, buy_rate: 210.39, sell_rate: 214.63 },
        Stock { name: "DOCN".to_string(), quantity: 423, price: 365.84, buy_rate: 362.18, sell_rate: 369.50 },
        Stock { name: "ZS".to_string(), quantity: 257, price: 279.49, buy_rate: 276.69, sell_rate: 282.28 },
        Stock { name: "BA".to_string(), quantity: 132, price: 180.99, buy_rate: 179.18, sell_rate: 182.80 },
        Stock { name: "GOOGL".to_string(), quantity: 209, price: 290.15, buy_rate: 287.25, sell_rate: 293.05 },
        Stock { name: "MS".to_string(), quantity: 150, price: 140.5, buy_rate: 139.09, sell_rate: 141.91 },
    ];
    Arc::new(Mutex::new(stocks))
}


// Function to add quantity to an existing stock in the vector
pub fn add_stock(stock_vector: &Arc<Mutex<Vec<Stock>>>, stock_name: &str, quantity: u32) {
    let mut stocks = stock_vector.lock().unwrap(); // Lock the Mutex to gain access to the vector

    if let Some(stock) = stocks.iter_mut().find(|s| s.name == stock_name) { // Find the stock by name
        stock.quantity += quantity; // Add the specified quantity to the stock
        // println!(
        //     "Received {} of {}. Updated quantity: {}",
        //     quantity, stock_name, stock.quantity
        // );
    } else {
        // println!(
        //     "Stock {} not found. Cannot add quantity as the stock doesn't exist.",
        //     stock_name
        // );
    }
}

// Function to deduct stock from the vector
pub fn deduct_stock(stock_vector: &Arc<Mutex<Vec<Stock>>>, stock_name: &str, quantity: u32) {
    let mut stocks = stock_vector.lock().unwrap();  // Lock the Mutex to gain access to the vector
    if let Some(stock) = stocks.iter_mut().find(|s| s.name == stock_name) {  // Find the stock by name
        if stock.quantity >= quantity {  // Check if there's enough stock to deduct
            stock.quantity -= quantity;  // Deduct the quantity from the stock
            //println!("Deducted {} of {}. New quantity: {}", quantity, stock_name, stock.quantity);
        } else {
            //println!("Not enough stock of {} to deduct. Current quantity: {}", stock_name, stock.quantity);
        }
    } else {
        //println!("Stock {} not found", stock_name);  // If stock not found, print a message
    }
}
use rand::Rng;

pub fn fluctuate_price(stock: &mut Stock) {
    let mut rng = rand::thread_rng();
    let fluctuation: f64 = rng.gen_range(-0.05..0.05); // Random fluctuation between -5% and 5%
    let percentage_change = fluctuation * 100.0; // Convert fluctuation to percentage
    let price_change = stock.price * fluctuation;

    // Print the fluctuation rate
    if fluctuation > 0.0 {
        // println!(
        //     "Stock price for {} will increase by {:+.2}%",
        //     stock.name, percentage_change
        // );
    } else {
        // println!(
        //     "Stock price for {} will decrease by {:+.2}%",
        //     stock.name, percentage_change
        // );
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


