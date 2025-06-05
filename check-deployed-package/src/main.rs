use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== logdash SDK package check ===");

    let api_key = env::var("LOGDASH_API_KEY").ok();
    let logs_seed = env::var("LOGS_SEED").unwrap_or_else(|_| "default".to_string());
    let metrics_seed = env::var("METRICS_SEED").unwrap_or_else(|_| "1".to_string());
    
    println!("Using API Key: {:?}", api_key);
    println!("Using Logs Seed: {}", logs_seed);
    println!("Using Metrics Seed: {}", metrics_seed);

    // Create logdash instance
    let (logger, metrics) = if let Some(key) = api_key {
        logdash::create_logdash(logdash::Config::default().api_key(key))
    } else {
        logdash::create_logdash(logdash::Config::default())
    };

    // Log messages with seed appended
    logger.info(&format!("This is an info log {}", logs_seed));
    logger.error(&format!("This is an error log {}", logs_seed));
    logger.warn(&format!("This is a warning log {}", logs_seed));
    logger.debug(&format!("This is a debug log {}", logs_seed));
    logger.http(&format!("This is a http log {}", logs_seed));
    logger.silly(&format!("This is a silly log {}", logs_seed));
    logger.info(&format!("This is an info log {}", logs_seed));
    logger.verbose(&format!("This is a verbose log {}", logs_seed));

    // Set and mutate metrics with seed
    let metrics_seed_value: f64 = metrics_seed.parse().unwrap_or(1.0);
    metrics.set("users".to_string(), metrics_seed_value);
    metrics.mutate("users".to_string(), 1.0);

    // Wait to ensure data is sent
    thread::sleep(Duration::from_secs(2));
} 