use crate::{log::LogMessage, metric::MetricMessage, propagator::Propagator};
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

pub struct Dispatch {
    propagator: Arc<dyn Propagator>,
}

impl Debug for Dispatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dispatch").finish()
    }
}

pub static DISPATCH: OnceLock<Dispatch> = OnceLock::new();

pub fn init_dispatch(collector: Arc<dyn Propagator>) {
    DISPATCH
        .set(Dispatch::new(collector))
        .expect("Dispatch already initialized");
}

impl Dispatch {
    fn new(propagator: Arc<dyn Propagator>) -> Self {
        Self { propagator }
    }

    #[inline]
    pub fn dispatch_log(&self, msg: LogMessage) {
        self.propagator.propagate_log(msg);
    }

    #[inline]
    pub fn dispatch_metric(&self, msg: MetricMessage) {
        self.propagator.propagate_metric(msg);
    }
}
