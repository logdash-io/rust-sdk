use crate::dispatch::DISPATCH;
use serde::Serialize;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use time::OffsetDateTime;

pub struct LogCollector {
    sequence_number: AtomicU64,
}

impl LogCollector {
    pub(crate) fn new() -> Self {
        Self {
            sequence_number: AtomicU64::new(0),
        }
    }
    #[inline(always)]
    fn send(&self, msg: LogMessage) {
        DISPATCH.get().unwrap().dispatch_log(msg);
    }

    pub fn error(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Error,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn warn(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Warn,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn info(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Info,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn http(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Http,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn verbose(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Verbose,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn debug(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Debug,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }

    pub fn silly(&self, message: impl IntoLog) {
        self.send(LogMessage {
            level: LogLevel::Silly,
            message: message.into_log(),
            created_at: OffsetDateTime::now_utc(),
            sequence_number: self.sequence_number.fetch_add(1, Ordering::SeqCst),
        });
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessage {
    pub message: String,
    pub level: LogLevel,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub sequence_number: u64,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Http,
    Verbose,
    Debug,
    Silly,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Http => write!(f, "http"),
            LogLevel::Verbose => write!(f, "verbose"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Silly => write!(f, "silly"),
        }
    }
}

pub trait IntoLog {
    fn into_log(self) -> String;
}

impl<T: ToString> IntoLog for T {
    fn into_log(self) -> String {
        self.to_string()
    }
}
