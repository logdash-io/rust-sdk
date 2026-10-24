use super::{MessageType, Propagator, PropagatorConfig};
use std::{
    marker::PhantomData,
    sync::mpsc::{Receiver, SyncSender, channel, sync_channel},
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

// Sending fails only when the worker thread is gone: drop the message rather than crash the host app.
impl<T: WorkerJob + Send + Sync + 'static> Propagator for Worker<T> {
    fn propagate_log(&self, msg: crate::log::LogMessage) {
        let _ = self.sender.send(MessageType::Log(msg));
    }

    fn propagate_metric(&self, msg: crate::metric::MetricMessage) {
        let _ = self.sender.send(MessageType::Metric(msg));
    }

    fn propagate_event(&self, msg: crate::event::EventMessage) {
        let _ = self.sender.send(MessageType::Event(msg));
    }

    // The worker sends messages in order, so its answer means everything queued before is sent.
    // A worker that is gone drops the answer channel, which ends the wait too.
    fn flush(&self) {
        let (flushed, answer) = channel();
        if self.sender.send(MessageType::Flush(flushed)).is_ok() {
            let _ = answer.recv();
        }
    }
}
