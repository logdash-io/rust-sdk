use super::{MessageType, Propagator, PropagatorConfig};
use crate::{log::LogMessage, metric::MetricMessage};
use owo_colors::OwoColorize;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

pub struct TerminalPropagator {
    sender: SyncSender<MessageType>,
}

impl TerminalPropagator {
    pub fn new(config: PropagatorConfig) -> Self {
        let (tx, rx) = sync_channel(30);
        thread::Builder::new()
            .name("logdash-propagator".into())
            .spawn(move || Self::thread_fn(config, rx))
            .unwrap();
        Self { sender: tx }
    }

    fn thread_fn(_cfg: PropagatorConfig, rx: Receiver<MessageType>) {
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    MessageType::Log(log) => match log.level {
                        crate::log::LogLevel::Error => {
                            println!(
                                "{} [{}] {}",
                                "Error:".truecolor(231, 0, 11),
                                log.created_at,
                                log.message
                            )
                        }
                        crate::log::LogLevel::Warn => {
                            println!(
                                "{} [{}] {}",
                                "Warn:".truecolor(254, 154, 0),
                                log.created_at,
                                log.message
                            )
                        }
                        crate::log::LogLevel::Info => {
                            println!(
                                "{} [{}] {}",
                                "Info:".truecolor(21, 93, 252),
                                log.created_at.to_utc(),
                                log.message
                            )
                        }
                        crate::log::LogLevel::Http => {
                            println!(
                                "{} [{}] {}",
                                "Http:".truecolor(0, 166, 166),
                                log.created_at,
                                log.message
                            )
                        }
                        crate::log::LogLevel::Verbose => {
                            println!(
                                "{} [{}] {}",
                                "Verbose:".truecolor(0, 166, 0),
                                log.created_at,
                                log.message
                            )
                        }
                        crate::log::LogLevel::Debug => {
                            println!(
                                "{} [{}] {}",
                                "Debug:".truecolor(0, 166, 62),
                                log.created_at.to_utc(),
                                log.message
                            )
                        }
                        crate::log::LogLevel::Silly => {
                            println!(
                                "{} [{}] {}",
                                "Silly:".truecolor(80, 80, 80),
                                log.created_at,
                                log.message
                            )
                        }
                    },
                    MessageType::Metric(metric) => {
                        println!(
                            "Metric: {} = {} {}",
                            metric.name, metric.value, metric.operation
                        );
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

impl Propagator for TerminalPropagator {
    fn propagate_log(&self, msg: LogMessage) {
        self.sender.send(MessageType::Log(msg)).unwrap();
    }

    fn propagate_metric(&self, msg: MetricMessage) {
        self.sender.send(MessageType::Metric(msg)).unwrap();
    }
}
