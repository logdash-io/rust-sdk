fn main() {
    // Create a new Logdash client with default configuration
    let (l, m) = logdash::create_logdash(logdash::Config::default().verbose(true));

    // Send an info log message
    l.info("info");
    l.warn("warn");
    l.error("error");
    l.debug("debug");
    l.silly("silly");
    l.http("http");
    l.verbose("verbose");

    // Send a metric message
    m.set("user".into(), 0.0);

    // Sleep for 10 seconds to allow the log message to be sent
    // This is just for demonstration purposes
    // In a real application, you would not want to sleep the thread like this
    std::thread::sleep(std::time::Duration::from_secs(10));
}
