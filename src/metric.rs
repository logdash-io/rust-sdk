use crate::dispatch::DISPATCH;
use serde::Serialize;
pub struct MetricCollector(());

impl MetricCollector {
    pub(crate) fn new() -> Self {
        Self(())
    }

    #[inline(always)]
    fn send(&self, msg: MetricMessage) {
        DISPATCH.get().unwrap().dispatch_metric(msg);
    }

    pub fn set(&self, name: String, value: f64) {
        self.send(MetricMessage {
            name,
            value,
            operation: MetricOperation::Set,
        });
    }

    pub fn mutate(&self, name: String, value: f64) {
        self.send(MetricMessage {
            name,
            value,
            operation: MetricOperation::Change,
        });
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricMessage {
    pub name: String,
    pub value: f64,
    pub operation: MetricOperation,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MetricOperation {
    Set,
    Change,
}

impl std::fmt::Display for MetricOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricOperation::Set => write!(f, "set"),
            MetricOperation::Change => write!(f, "change"),
        }
    }
}
