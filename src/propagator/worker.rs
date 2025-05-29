use super::{MessageType, Propagator, PropagatorConfig};
use std::{
    marker::PhantomData,
    sync::mpsc::{Receiver, SyncSender, sync_channel},
};
pub trait WorkerJob {
    fn job(cfg: PropagatorConfig, rx: Receiver<MessageType>);
}

pub struct Worker<P: WorkerJob> {
    sender: SyncSender<MessageType>,
    _propagator: PhantomData<P>,
}

impl<T: WorkerJob> Worker<T> {
    pub fn new(config: PropagatorConfig) -> Self {
        let (tx, rx) = sync_channel(30);
        std::thread::Builder::new()
            .name("logdash-worker".into())
            .spawn(move || T::job(config, rx))
            .unwrap();
        Self {
            sender: tx,
            _propagator: PhantomData,
        }
    }
}

impl<T: WorkerJob + Send + Sync + 'static> Propagator for Worker<T> {
    fn propagate_log(&self, msg: crate::log::LogMessage) {
        self.sender.send(MessageType::Log(msg)).unwrap();
    }

    fn propagate_metric(&self, msg: crate::metric::MetricMessage) {
        self.sender.send(MessageType::Metric(msg)).unwrap();
    }
}
