//! In-memory map of identified peers.

use std::collections::HashMap;
use std::sync::Mutex;

/// Identification fields kept for one accepted peer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionInfo {
    pub operating_system: String,
    pub hostname: String,
    pub username: String,
}

#[derive(Default)]
struct Inner {
    next_id: u64,
    by_id: HashMap<u64, SessionInfo>,
}

/// Map from connection id to identification fields.
#[derive(Default)]
pub struct Sessions {
    inner: Mutex<Inner>,
}

impl Sessions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores `info` and returns the new connection id.
    pub fn insert(&self, info: SessionInfo) -> u64 {
        let mut inner = self.inner.lock().expect("sessions");
        let id = inner.next_id;
        inner.next_id = id + 1;
        inner.by_id.insert(id, info);
        id
    }

    /// Removes the entry for `id` if it is present.
    pub fn remove(&self, id: u64) {
        self.inner.lock().expect("sessions").by_id.remove(&id);
    }

    /// True when `id` is still in the map.
    pub fn contains(&self, id: u64) -> bool {
        self.inner.lock().expect("sessions").by_id.contains_key(&id)
    }

    /// Number of identified peers.
    pub fn len(&self) -> usize {
        self.inner.lock().expect("sessions").by_id.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_remove() {
        let sessions = Sessions::new();
        let id = sessions.insert(SessionInfo {
            operating_system: "FakeOS".to_string(),
            hostname: "fake-host".to_string(),
            username: "fake-user".to_string(),
        });
        assert!(sessions.contains(id));
        assert_eq!(sessions.len(), 1);

        sessions.remove(id);
        assert!(!sessions.contains(id));
        assert_eq!(sessions.len(), 0);
    }
}
