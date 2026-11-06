//! Length-prefix framing and the identification messages.

pub use prost::Message;

pub mod codec;
pub mod error;

pub mod identification {
    include!(concat!(env!("OUT_DIR"), "/ekranoplan.protocol.rs"));
}
