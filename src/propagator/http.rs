use super::{MessageType, Propagator, PropagatorConfig};
use crate::{log::LogMessage, metric::MetricMessage};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

pub struct HttpPropagator {
    // config: PropagatorConfig,
    sender: SyncSender<MessageType>,
}

impl HttpPropagator {
    pub fn new(config: PropagatorConfig) -> Self {
        let (tx, rx) = sync_channel(30);
        thread::Builder::new()
            .name("logdash-propagator".into())
            .spawn(move || Self::thread_fn(config, rx))
            .unwrap();
        Self { sender: tx }
    }

    fn thread_fn(cfg: PropagatorConfig, rx: Receiver<MessageType>) {
        let log_url = format!("{}/logs", cfg.api_url);
        let metric_url = format!("{}/metrics", cfg.api_url);
        let api_key = cfg.api_key.unwrap();
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    MessageType::Log(log) => {
                        ureq::post(&log_url)
                            .header("project-api-key", &api_key)
                            .send_json(log)
                            .unwrap();
                        if cfg.verbose {
                            println!("Log sent",);
                        }
                    }
                    MessageType::Metric(metric) => {
                        ureq::put(&metric_url)
                            .header("project-api-key", &api_key)
                            .send_json(metric)
                            .unwrap();
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

impl Propagator for HttpPropagator {
    fn propagate_log(&self, msg: LogMessage) {
        self.sender.send(MessageType::Log(msg)).unwrap();
    }

    fn propagate_metric(&self, msg: MetricMessage) {
        self.sender.send(MessageType::Metric(msg)).unwrap();
    }
}
