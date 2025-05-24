pub mod http;
pub mod terminal;
use std::sync::Arc;

use crate::{log::LogMessage, metric::MetricMessage};

#[derive(Debug, Clone)]
pub struct PropagatorConfig {
    pub(crate) api_key: Option<String>,
    api_url: String,
    verbose: bool,
}

impl Default for PropagatorConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("LOGDASH_API_KEY").ok(),
            api_url: "https://api.logdash.io".to_string(),
            verbose: false,
        }
    }
}

impl PropagatorConfig {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }
    pub fn api_url(mut self, api_url: String) -> Self {
        self.api_url = api_url;
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    #[inline]
    pub(crate) fn local(&self) -> bool {
        self.api_key.is_none()
    }
}

pub enum MessageType {
    Log(LogMessage),
    Metric(MetricMessage),
}

pub trait Propagator: Send + Sync + 'static {
    fn propagate_log(&self, msg: LogMessage);
    fn propagate_metric(&self, msg: MetricMessage);
}

impl Propagator for Arc<dyn Propagator> {
    fn propagate_log(&self, msg: LogMessage) {
        self.as_ref().propagate_log(msg);
    }

    fn propagate_metric(&self, msg: MetricMessage) {
        self.as_ref().propagate_metric(msg);
    }
}
