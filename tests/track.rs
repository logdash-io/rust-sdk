mod common;

use serde_json::json;
use std::time::Duration;
use uuid::{Uuid, Version};

#[test]
fn track_posts_valid_events_to_the_events_endpoint() {
    let (api_url, requests) = common::api();
    logdash::create_logdash(
        logdash::Config::default()
            .api_key("key".into())
            .api_url(api_url)
            .app("desktop".into()),
    );

    logdash::track("Agent Run", &[], None);
    logdash::track("agent_run", &[], Some("ada@"));
    logdash::track(
        "agent_run",
        &[
            ("agent", "support".into()),
            ("steps", 4.into()),
            ("success", true.into()),
        ],
        Some("42"),
    );
    logdash::track("app_opened", &[], None);

    // The worker sends events in order, so a dropped one would have arrived first.
    let next_request = || requests.recv_timeout(Duration::from_secs(5)).unwrap();
    let mut agent_run = next_request();
    assert!(agent_run.line.starts_with("POST /events "));
    assert!(agent_run.headers.contains(&"project-api-key: key".into()));
    assert!(
        agent_run
            .headers
            .contains(&"content-type: application/json; charset=utf-8".into())
    );
    let agent_run_id = agent_run
        .body
        .as_object_mut()
        .unwrap()
        .remove("id")
        .unwrap();
    assert_eq!(
        Uuid::parse_str(agent_run_id.as_str().unwrap())
            .unwrap()
            .get_version(),
        Some(Version::Random)
    );
    assert_eq!(
        agent_run.body,
        json!({
            "name": "agent_run",
            "props": { "agent": "support", "steps": 4, "success": true },
            "userId": "42",
            "app": "desktop"
        })
    );

    let mut app_opened = next_request();
    let app_opened_id = app_opened.body.as_object_mut().unwrap().remove("id");
    assert_ne!(app_opened_id, Some(agent_run_id));
    assert_eq!(
        app_opened.body,
        json!({ "name": "app_opened", "app": "desktop" })
    );
}
