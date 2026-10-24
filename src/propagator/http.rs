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
                        ureq::post(&log_url)
                            .header("project-api-key", &api_key)
                            .send_json(Body { msg: &log, app })
                            .unwrap();
                        if cfg.verbose {
                            println!("Log send: {} {}", log.level, log.message);
                        }
                    }
                    MessageType::Metric(metric) => {
                        ureq::put(&metric_url)
                            .header("project-api-key", &api_key)
                            .send_json(Body { msg: &metric, app })
                            .unwrap();
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
    use super::Body;
    use crate::log::{LogLevel, LogMessage};
    use crate::metric::{MetricMessage, MetricOperation};
    use serde_json::{Value, json};
    use time::OffsetDateTime;

    fn log_body(app: Option<&str>) -> Value {
        let log = LogMessage {
            message: "Desktop app started".into(),
            level: LogLevel::Info,
            created_at: OffsetDateTime::UNIX_EPOCH,
            sequence_number: 0,
        };
        serde_json::to_value(Body { msg: &log, app }).unwrap()
    }

    fn metric_body(app: Option<&str>) -> Value {
        let metric = MetricMessage {
            name: "users".into(),
            value: 1.0,
            operation: MetricOperation::Set,
        };
        serde_json::to_value(Body { msg: &metric, app }).unwrap()
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
