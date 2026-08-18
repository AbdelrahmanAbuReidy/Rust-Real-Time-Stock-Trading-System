use rand::prelude::*; // For random number generation
use rand::random;

#[derive(Debug, Clone)]
pub struct Stock {
    pub name: String,
    pub quantity: u32,
    pub price: f64, // The price of the stock (market price)
    pub buy_rate: f64,
    pub sell_rate: f64,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: u32,
    pub max_price: f64,
    pub stocks: Vec<Stock>,
    pub action: String,  // Add action field to the Client struct
}

impl Client {
    // Constructor for the Client struct
    pub fn new(id: u32, stocks: Vec<Stock>, max_price: f64, action:String ) -> Self {
        Client { id, stocks, max_price, action}
    }
}

pub fn generate_client(id: &mut u32, stocks: &[Stock]) -> Option<Client> {
    let mut rng = thread_rng();

    // Ensure there's at least one stock to choose from
    if stocks.is_empty() {
        println!("No stocks available to choose from.");
        return None;
    }

    // Choose a random number between 1 and 3 for the number of stocks the client will have
    let num_stocks = rng.gen_range(1..=3);

    let action = if random::<bool>() { "Buy" } else { "Sell" };

    // Randomly select 'num_stocks' from the available stocks
    let selected_stocks: Vec<Stock> = stocks.choose_multiple(&mut rng, num_stocks).cloned().collect();

    // Randomly generate the stock quantity between 1 and 5 for each selected stock
    let stocks_with_quantities: Vec<Stock> = selected_stocks.into_iter().map(|mut stock| {
        stock.quantity = rng.gen_range(1..=5);
        stock
    }).collect();

    // Generate a random max price for the client between 100.0 and 500.0
    let random_max_price: f64 = rng.gen_range(1000.0..=5000.0);

// Round the random max price to two decimal places
let max_price_rounded = (random_max_price * 100.0).round() / 100.0;

    // Create a new Client with the selected stocks and random max price
    let client = Client::new(*id, stocks_with_quantities, max_price_rounded, action.to_string());

    // Increment the ID
    *id += 1;

    Some(client)
}
