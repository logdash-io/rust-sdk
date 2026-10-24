use crate::metric::MetricOperation;

use super::{
    MessageType, PropagatorConfig,
    worker::{Worker, WorkerJob},
};
use serde::Serialize;
use std::sync::mpsc::Receiver;
pub struct HttpPropagator {
    _priv: (),
}

/// JSON body of a log or metric, with the app that sent it when one is set.
#[derive(Serialize)]
struct Body<'a, T> {
    #[serde(flatten)]
    msg: &'a T,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<&'a str>,
}

pub fn http(cfg: PropagatorConfig) -> Worker<HttpPropagator> {
    Worker::<HttpPropagator>::new(cfg)
}

impl WorkerJob for HttpPropagator {
    fn job(cfg: PropagatorConfig, rx: Receiver<MessageType>) {
        let log_url = format!("{}/logs", cfg.api_url);
        let metric_url = format!("{}/metrics", cfg.api_url);
        let api_key = cfg.api_key.unwrap();
        let app = cfg.app.as_deref();
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    MessageType::Log(log) => {
                        if let Err(e) = ureq::post(&log_url)
                            .header("project-api-key", &api_key)
                            .send_json(Body { msg: &log, app })
                        {
                            eprintln!("Error sending log: {}", e);
                            continue;
                        }
                        if cfg.verbose {
                            println!("Log send: {} {}", log.level, log.message);
                        }
                    }
                    MessageType::Metric(metric) => {
                        if let Err(e) = ureq::put(&metric_url)
                            .header("project-api-key", &api_key)
                            .send_json(Body { msg: &metric, app })
                        {
                            eprintln!("Error sending metric: {}", e);
                            continue;
                        }
                        if cfg.verbose {
                            match metric.operation.clone() {
                                MetricOperation::Set => {
                                    println!("Metric set: {} = {}", metric.name, metric.value);
                                }

                                MetricOperation::Change => {
                                    println!("Metric changed: {} = {}", metric.name, metric.value);
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Body, http};
    use crate::log::{LogLevel, LogMessage};
    use crate::metric::{MetricMessage, MetricOperation};
    use crate::propagator::{Propagator, PropagatorConfig};
    use serde_json::{Value, json};
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    use time::OffsetDateTime;

    fn log() -> LogMessage {
        LogMessage {
            message: "Desktop app started".into(),
            level: LogLevel::Info,
            created_at: OffsetDateTime::UNIX_EPOCH,
            sequence_number: 0,
        }
    }

    fn metric() -> MetricMessage {
        MetricMessage {
            name: "users".into(),
            value: 1.0,
            operation: MetricOperation::Set,
        }
    }

    fn log_body(app: Option<&str>) -> Value {
        serde_json::to_value(Body { msg: &log(), app }).unwrap()
    }

    fn metric_body(app: Option<&str>) -> Value {
        serde_json::to_value(Body {
            msg: &metric(),
            app,
        })
        .unwrap()
    }

    /// Reads one request and rejects it with 400, like the API does for a body it refuses.
    /// Returns the request line.
    fn reject(stream: TcpStream) -> String {
        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        let mut content_length = 0;
        let mut header = String::new();
        while reader.read_line(&mut header).unwrap() > 2 {
            if let Some(value) = header.to_ascii_lowercase().strip_prefix("content-length:") {
                content_length = value.trim().parse().unwrap();
            }
            header.clear();
        }
        reader.read_exact(&mut vec![0; content_length]).unwrap();
        (&stream)
            .write_all(
                b"HTTP/1.1 400 Bad Request\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
            )
            .unwrap();
        request_line
    }

    #[test]
    fn worker_keeps_sending_after_a_failed_request() {
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_url = format!("http://{}", server.local_addr().unwrap());
        let (requests, received) = mpsc::channel();
        thread::spawn(move || {
            for stream in server.incoming() {
                requests.send(reject(stream.unwrap())).unwrap();
            }
        });

        let worker = http(
            PropagatorConfig::default()
                .api_key("key".into())
                .api_url(api_url),
        );
        worker.propagate_log(log());
        worker.propagate_metric(metric());

        let next_request = || received.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(next_request().starts_with("POST /logs "));
        assert!(next_request().starts_with("PUT /metrics "));
    }

    #[test]
    fn log_carries_app_when_set() {
        assert_eq!(
            log_body(Some("desktop")),
            json!({
                "message": "Desktop app started",
                "level": "info",
                "createdAt": "1970-01-01T00:00:00Z",
                "sequenceNumber": 0,
                "app": "desktop"
            })
        );
    }

    #[test]
    fn log_has_no_app_key_when_unset() {
        assert_eq!(
            log_body(None),
            json!({
                "message": "Desktop app started",
                "level": "info",
                "createdAt": "1970-01-01T00:00:00Z",
                "sequenceNumber": 0
            })
        );
    }

    #[test]
    fn metric_carries_app_when_set() {
        assert_eq!(
            metric_body(Some("desktop")),
            json!({ "name": "users", "value": 1.0, "operation": "set", "app": "desktop" })
        );
    }

    #[test]
    fn metric_has_no_app_key_when_unset() {
        assert_eq!(
            metric_body(None),
            json!({ "name": "users", "value": 1.0, "operation": "set" })
        );
    }
}
