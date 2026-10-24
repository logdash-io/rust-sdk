fn main() {
    // Create a new Logdash client with default configuration
    let (l, m) = logdash::create_logdash(
        logdash::Config::default()
            .api_key("Your Api Key".into())
            .verbose(true),
    );

    // Send an info log message
    l.info("Rust SDK example");

    // Send a metric message
    m.set("user".into(), 0.0);

    // Wait until the log and the metric are sent before the program exits
    logdash::flush();
}
