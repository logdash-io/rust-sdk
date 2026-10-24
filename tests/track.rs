use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uuid::{Uuid, Version};

/// A request the fake API received.
struct Request {
    line: String,
    headers: Vec<String>,
    body: Value,
}

/// Starts a fake API that accepts every request with 202, like `POST /events` does.
/// Returns its URL and the requests it receives.
fn api() -> (String, mpsc::Receiver<Request>) {
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", server.local_addr().unwrap());
    let (requests, received) = mpsc::channel();
    thread::spawn(move || {
        for stream in server.incoming() {
            let stream = stream.unwrap();
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let mut headers = Vec::new();
            loop {
                let mut header = String::new();
                reader.read_line(&mut header).unwrap();
                match header.trim_end() {
                    "" => break,
                    header => headers.push(header.to_ascii_lowercase()),
                }
            }
            let length = headers
                .iter()
                .find_map(|header| header.strip_prefix("content-length:"))
                .map_or(0, |length| length.trim().parse().unwrap());
            let mut body = vec![0; length];
            reader.read_exact(&mut body).unwrap();
            (&stream)
                .write_all(
                    b"HTTP/1.1 202 Accepted\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
                )
                .unwrap();
            let body = serde_json::from_slice(&body).unwrap();
            requests
                .send(Request {
                    line,
                    headers,
                    body,
                })
                .unwrap();
        }
    });
    (url, received)
}

#[test]
fn track_posts_valid_events_to_the_events_endpoint() {
    let (api_url, requests) = api();
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
