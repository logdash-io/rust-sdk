mod common;

#[test]
fn flush_waits_until_queued_logs_metrics_and_events_are_sent() {
    // Nothing is queued before create_logdash, so this returns at once.
    logdash::flush();

    let (api_url, requests) = common::api();
    let (logger, metrics) = logdash::create_logdash(
        logdash::Config::default()
            .api_key("key".into())
            .api_url(api_url),
    );
    logger.info("Desktop app closing");
    metrics.set("users".into(), 1.0);
    logdash::track("app_closed", &[], None);

    logdash::flush();

    let sent: Vec<_> = requests
        .try_iter()
        .map(|request| (request.line, request.body))
        .collect();
    assert_eq!(sent.len(), 3);
    assert!(sent[0].0.starts_with("POST /logs "));
    assert_eq!(sent[0].1["message"], "Desktop app closing");
    assert!(sent[1].0.starts_with("PUT /metrics "));
    assert_eq!(sent[1].1["name"], "users");
    assert!(sent[2].0.starts_with("POST /events "));
    assert_eq!(sent[2].1["name"], "app_closed");
}
