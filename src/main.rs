pub mod stock_generator;
pub mod stock_inventory;
pub mod stock_price;
pub mod order_management;

use amiquip::{Connection, Result};
use stock_generator::StockTracker;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use chrono::Local; 
use perf_monitor::cpu::{ThreadStat, ProcessStat, processor_numbers};
use perf_monitor::io::get_process_io_stats;
use perf_monitor::mem::get_process_memory_info;
use cpu_time::{ProcessTime, ThreadTime};
use cpu_monitor::CpuInstant;
use std::io::{self};

// Helper function to print messages with a timestamp
fn log_with_timestamp(message: &str) {
    let now = Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message);
}


pub fn cpu_mon() -> Result<(), io::Error> {
    // Capture the start CPU instant
    let start: CpuInstant = CpuInstant::now()?;

    // Simulate work or pause to allow for CPU measurement
    std::thread::sleep(Duration::from_millis(100));  // Adjust time as needed

    // Capture the end CPU instant
    let end: CpuInstant = CpuInstant::now()?;

    // Calculate the CPU usage duration
    let duration = end - start;

    // Calculate non-idle CPU usage as a percentage
    println!("CPU Usage: {:.0}%", duration.non_idle() * 100.);

    Ok(())
}

// Performance monitoring function
fn perf_mon() {
    // CPU
    let core_num: usize = processor_numbers().unwrap();
    let mut stat_p: ProcessStat = ProcessStat::cur().unwrap();
    let mut stat_t: ThreadStat = ThreadStat::cur().unwrap();
    let _ = (0..1_000).into_iter().sum::<i128>(); // Simulate some work
    let usage_p: f64 = stat_p.cpu().unwrap() * 100f64;
    let usage_t: f64 = stat_t.cpu().unwrap() * 100f64;

    println!(
        "[CPU] Core Number: {}, Process Usage: {:.2}%, Current Thread Usage: {:.2}%",
        core_num, usage_p, usage_t
    );

    // Memory
    let mem_info = get_process_memory_info().unwrap();
    println!(
        "[Memory] Memory Used: {} bytes, Virtual Memory Used: {} bytes",
        mem_info.resident_set_size, mem_info.virtual_memory_size
    );

    // IO
    let io_stat = get_process_io_stats().unwrap();
    println!(
        "[IO] IO-In: {} bytes, IO-Out: {} bytes",
        io_stat.read_bytes, io_stat.write_bytes
    );
}

// CPU time monitoring function
fn cpu_time() {
    // Measure process time
    let start_process: ProcessTime = ProcessTime::try_now().expect("Getting process time failed");
    let start_thread: ThreadTime = ThreadTime::try_now().expect("Getting thread time failed");

    // Simulate some work
    thread::sleep(Duration::from_millis(500));

    // Calculate elapsed CPU time
    let elapsed_process: Duration = start_process.try_elapsed().expect("Getting process time failed");
    let elapsed_thread: Duration = start_thread.try_elapsed().expect("Getting thread time failed");

    println!("Process CPU Time: {:?}", elapsed_process);
    println!("Thread CPU Time: {:?}", elapsed_thread);
}

fn main() -> Result<()> {
    let runtime = Duration::from_secs(90); // Run for 90 seconds
    let start_time = Instant::now();

    // Set up RabbitMQ connection
    let amqp_url = std::env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());
    let mut connection = Connection::insecure_open(&amqp_url)?;

    // Create separate channels for sending and receiving
    let send_channel = connection.open_channel(None)?;
    let receive_channel = connection.open_channel(None)?;

    // Shared channels for safe concurrency
    let shared_send_channel = Arc::new(Mutex::new(send_channel));
    let shared_receive_channel = Arc::new(Mutex::new(receive_channel));

    // Shared stock vector
    let stock_vector = stock_generator::generate_stock_vector();

    // Initialize the stock tracker
    let tracker = Arc::new(Mutex::new(StockTracker::new()));

    log_with_timestamp("Stock Trading System starting...");

    {
        let stock_lock = stock_vector.lock().unwrap(); 
        for stock in stock_lock.iter() {
            println!("{:?}", stock);
        }
    }

    // Start a sender thread 
    let sender_channel = Arc::clone(&shared_send_channel);
    let stock_vector_clone = Arc::clone(&stock_vector);
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            stock_generator::send_stocks(sender_channel.clone(), stock_vector_clone.clone());
            log_with_timestamp("Sent stocks to the broker.");
            thread::sleep(Duration::from_secs(1)); // Send stocks every 1 seconds
        }
        log_with_timestamp("Sender thread stopping.");
    });

    // Start a receiver thread
    let receiver_channel = Arc::clone(&shared_receive_channel);
    let stock_vector_clone2 = Arc::clone(&stock_vector);
    let tracker_clone = Arc::clone(&tracker);
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            stock_inventory::receive_stocks(receiver_channel.clone(), stock_vector_clone2.clone(), tracker_clone.clone());
            thread::sleep(Duration::from_secs(1));  // Receive stocks every 3 seconds
        }
        log_with_timestamp("Receiver thread stopping.");
    });

    // Start a price fluctuation thread
    let stock_vector_clone3 = Arc::clone(&stock_vector);
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            stock_price::update_stock_prices(stock_vector_clone3.clone());
            thread::sleep(Duration::from_secs(5)); // Fluctuate price every 5 seconds
        }
        log_with_timestamp("Price fluctuation thread stopping.");
    });

    // Start CPU usage monitoring thread
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            if let Err(e) = cpu_mon() {
                eprintln!("Error monitoring CPU: {}", e);
            }
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before checking again
        }
    });

    // Start performance monitoring thread
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            perf_mon();
            thread::sleep(Duration::from_secs(5)); // Monitor every 5 seconds
        }
    });

    // Start CPU time monitoring thread
    thread::spawn(move || {
        while Instant::now().duration_since(start_time) < runtime {
            cpu_time();
            thread::sleep(Duration::from_secs(5)); // Monitor every 5 seconds
        }
    });

    // Keep the program running for 95 seconds
    while Instant::now().duration_since(start_time) < Duration::from_secs(95) {
        thread::sleep(Duration::from_secs(1));
    }

    // Generate the final report 
    log_with_timestamp("Generating final report...");

    {
        // Generate the stock report
        let tracker = tracker.lock().unwrap();
        tracker.generate_report(); 
    }

    let _ = connection.close();
    log_with_timestamp("Stock Trading System completed.");

    Ok(())
}
