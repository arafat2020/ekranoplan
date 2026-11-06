use protocol::codec::{decode, encode, MAX_PAYLOAD_LEN};
use protocol::error::CodecError;

#[test]
fn round_trip_short_payload() {
    let payload = b"ekranoplan";
    let frame = encode(payload).unwrap();
    assert_eq!(decode(&frame).unwrap(), payload);
}

#[test]
fn round_trip_empty_payload() {
    let frame = encode(b"").unwrap();
    assert_eq!(frame, [0, 0, 0, 0]);
    assert_eq!(decode(&frame).unwrap(), b"");
}

#[test]
fn length_claims_more_bytes_than_the_buffer_holds() {
    let mut buf = 8u32.to_le_bytes().to_vec();
    buf.extend_from_slice(b"abc");

    assert_eq!(
        decode(&buf),
        Err(CodecError::Incomplete {
            claimed: 8,
            available: 3,
        })
    );
}

#[test]
fn length_above_five_megabytes() {
    let len = MAX_PAYLOAD_LEN + 1;
    let buf = len.to_le_bytes();

    assert_eq!(decode(&buf), Err(CodecError::TooLarge { len }));
}
