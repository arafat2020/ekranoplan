//! Length-prefix framing. A frame is a 4-byte little-endian length, then that many payload bytes.

use crate::error::CodecError;

/// Largest payload, in bytes. Matches the 5 MB cap on the existing client.
pub const MAX_PAYLOAD_LEN: u32 = 5 * 1024 * 1024;

const HEADER_LEN: usize = 4;

/// Writes `payload` as one frame.
///
/// A length of zero is a valid empty payload.
pub fn encode(payload: &[u8]) -> Result<Vec<u8>, CodecError> {
    let len = u32::try_from(payload.len()).map_err(|_| CodecError::TooLarge { len: u32::MAX })?;
    check_len(len)?;

    let mut frame = Vec::with_capacity(HEADER_LEN + payload.len());
    frame.extend_from_slice(&len.to_le_bytes());
    frame.extend_from_slice(payload);
    Ok(frame)
}

/// Reads one frame from `buf`.
///
/// `buf` must hold the header and every payload byte the header claims. Bytes after that payload are ignored.
pub fn decode(buf: &[u8]) -> Result<&[u8], CodecError> {
    if buf.len() < HEADER_LEN {
        return Err(CodecError::Incomplete {
            claimed: HEADER_LEN,
            available: buf.len(),
        });
    }

    let len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    check_len(len)?;

    let claimed = len as usize;
    let available = buf.len() - HEADER_LEN;
    if available < claimed {
        return Err(CodecError::Incomplete { claimed, available });
    }

    Ok(&buf[HEADER_LEN..HEADER_LEN + claimed])
}

fn check_len(len: u32) -> Result<(), CodecError> {
    if len > MAX_PAYLOAD_LEN {
        Err(CodecError::TooLarge { len })
    } else {
        Ok(())
    }
}
