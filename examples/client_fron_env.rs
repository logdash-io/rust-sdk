fn main() {
    // Create a new Logdash client with default configuration
    let (l, m) = logdash::create_logdash(logdash::Config::default().verbose(true));

    // Send an info log message
    l.info("Rust SDK example");
    l.warn("elo");
    l.error("elo");
    l.debug("elo");
    l.silly("elo");
    l.http("elo");
    l.verbose("elo");

    // Send a metric message
    m.set("user".into(), 0.0);

    // Wait until the logs and the metric are sent before the program exits
    logdash::flush();
}
