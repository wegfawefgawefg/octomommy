use std::sync::Arc;

use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOG_EVENT_QUEUE: Arc<ArrayQueue<LogEvent>> = Arc::new(ArrayQueue::new(1000));
}

pub enum LogEvent {
    Disconnect,
    Connect,
    ClientMessage,
    ServerMessage,
    // OriginSourceRegistration,
    // OriginSourceDeregistration,
}
