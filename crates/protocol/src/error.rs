use std::fmt;

/// A frame could not be written or read.
#[derive(Debug, PartialEq, Eq)]
pub enum CodecError {
    /// The declared payload length is above [`crate::codec::MAX_PAYLOAD_LEN`].
    TooLarge { len: u32 },
    /// `buf` does not contain the number of bytes the header claims.
    Incomplete { claimed: usize, available: usize },
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::TooLarge { len } => write!(f, "payload length {len} exceeds 5 MB"),
            CodecError::Incomplete { claimed, available } => {
                write!(f, "need {claimed} bytes, have {available}")
            }
        }
    }
}

impl std::error::Error for CodecError {}
