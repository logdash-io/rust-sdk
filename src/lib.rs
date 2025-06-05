mod dispatch;
mod log;
mod metric;
mod propagator;
pub use log::LogCollector;
pub use metric::MetricCollector;
use propagator::Propagator;
pub use propagator::PropagatorConfig as Config;
use std::sync::Arc;

pub fn create_logdash(cfg: Config) -> (LogCollector, MetricCollector) {
    if cfg.local() {
        let propagator: Arc<dyn Propagator> = Arc::new(propagator::terminal(cfg));
        dispatch::init_dispatch(propagator);
    } else {
        let propagator: Arc<dyn Propagator> = Arc::new(propagator::http(cfg));
        dispatch::init_dispatch(propagator);
    }
    (LogCollector::new(), MetricCollector::new())
}
