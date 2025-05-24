fn main() {
    // Create a new Logdash client with default configuration
    let (_l, _m) = logdash::create_logdash(logdash::Config::default().verbose(true));

    // Send an info log message
    _l.info("Rust SDK example");

    // Sleep for 10 seconds to allow the log message to be sent
    // This is just for demonstration purposes
    // In a real application, you would not want to sleep the thread like this
    std::thread::sleep(std::time::Duration::from_secs(10));
}
