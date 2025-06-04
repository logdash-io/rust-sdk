use super::{
    MessageType, PropagatorConfig,
    worker::{Worker, WorkerJob},
};
use std::sync::mpsc::Receiver;
pub struct HttpPropagator {
    _priv: (),
}

pub fn http(cfg: PropagatorConfig) -> Worker<HttpPropagator> {
    Worker::<HttpPropagator>::new(cfg)
}

impl WorkerJob for HttpPropagator {
    fn job(cfg: PropagatorConfig, rx: Receiver<MessageType>) {
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
