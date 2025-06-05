use super::{
    MessageType, PropagatorConfig,
    worker::{Worker, WorkerJob},
};
use owo_colors::OwoColorize;
use std::sync::mpsc::Receiver;

pub struct TerminalPropagator {
    _priv: (),
}

pub fn terminal(cfg: PropagatorConfig) -> Worker<TerminalPropagator> {
    Worker::<TerminalPropagator>::new(cfg)
}

impl WorkerJob for TerminalPropagator {
    fn job(_cfg: PropagatorConfig, rx: Receiver<MessageType>) {
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
                                log.created_at,
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
                                log.created_at,
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
