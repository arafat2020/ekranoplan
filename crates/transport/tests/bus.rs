use std::sync::{Arc, Mutex};

use transport::bus::{Bus, ClientIdentification, Processor, SessionId};

struct Bound {
    session: SessionId,
    hits: Mutex<u32>,
}

impl Processor for Bound {
    fn can_execute(&self, _message: &ClientIdentification) -> bool {
        true
    }

    fn can_execute_from(&self, session: SessionId) -> bool {
        session == self.session
    }

    fn execute(&self, _session: SessionId, _message: &ClientIdentification) {
        *self.hits.lock().expect("hits") += 1;
    }
}

fn identification() -> ClientIdentification {
    ClientIdentification {
        version: "0.1.0".to_string(),
        operating_system: "Linux".to_string(),
        account_type: "User".to_string(),
        country: "Bangladesh".to_string(),
        country_code: "BD".to_string(),
        image_index: 0,
        id: "id".to_string(),
        username: "arafat".to_string(),
        pc_name: "build".to_string(),
        tag: "phase-03".to_string(),
        encryption_key: String::new(),
        signature: Vec::new(),
        os: 3,
        capabilities: 0,
    }
}

#[test]
fn session_filter_and_unregister() {
    let session_a = SessionId(1);
    let session_b = SessionId(2);
    let message = identification();

    let processor_a = Arc::new(Bound {
        session: session_a,
        hits: Mutex::new(0),
    });
    let processor_b = Arc::new(Bound {
        session: session_b,
        hits: Mutex::new(0),
    });

    let bus = Bus::new();
    bus.register(processor_a.clone());
    bus.register(processor_b.clone());

    bus.process(session_a, &message);

    assert_eq!(*processor_a.hits.lock().expect("hits"), 1);
    assert_eq!(
        *processor_b.hits.lock().expect("hits"),
        0,
        "session B must not run a message from session A"
    );

    bus.unregister(&(processor_a.clone() as Arc<dyn Processor>));
    bus.process(session_a, &message);

    assert_eq!(
        *processor_a.hits.lock().expect("hits"),
        1,
        "an unregistered processor must not run"
    );
    assert_eq!(*processor_b.hits.lock().expect("hits"), 0);
}
