//! In-memory handler list. A call to `process` snapshots the matches, drops the lock, then runs them.

use std::sync::{Arc, Mutex};

pub use protocol::identification::ClientIdentification;

/// Which connection a message arrived on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionId(pub u64);

/// A handler that may accept a [`ClientIdentification`].
pub trait Processor: Send + Sync {
    fn can_execute(&self, message: &ClientIdentification) -> bool;
    fn can_execute_from(&self, session: SessionId) -> bool;
    fn execute(&self, session: SessionId, message: &ClientIdentification);
}

/// Registered processors. Not a socket.
#[derive(Default)]
pub struct Bus {
    processors: Mutex<Vec<Arc<dyn Processor>>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            processors: Mutex::new(Vec::new()),
        }
    }

    /// Adds `processor` unless this exact instance is already registered.
    pub fn register(&self, processor: Arc<dyn Processor>) {
        let mut processors = self.processors.lock().expect("bus lock");
        if processors
            .iter()
            .any(|existing| Arc::ptr_eq(existing, &processor))
        {
            return;
        }
        processors.push(processor);
    }

    /// Removes `processor` if it was registered.
    pub fn unregister(&self, processor: &Arc<dyn Processor>) {
        let mut processors = self.processors.lock().expect("bus lock");
        processors.retain(|existing| !Arc::ptr_eq(existing, processor));
    }

    /// Runs every processor that accepts both `message` and `session`.
    pub fn process(&self, session: SessionId, message: &ClientIdentification) {
        let matched: Vec<Arc<dyn Processor>> = {
            let processors = self.processors.lock().expect("bus lock");
            processors
                .iter()
                .filter(|processor| {
                    processor.can_execute(message) && processor.can_execute_from(session)
                })
                .cloned()
                .collect()
        };

        for processor in matched {
            processor.execute(session, message);
        }
    }
}
