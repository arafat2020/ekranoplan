use protocol::codec::{decode, encode};
use protocol::identification::{ClientIdentification, Os};
use protocol::Message;

#[test]
fn identification_round_trip_goes_through_the_frame() {
    let message = ClientIdentification {
        version: "0.1.0".to_string(),
        operating_system: "Linux".to_string(),
        account_type: "User".to_string(),
        country: "Bangladesh".to_string(),
        country_code: "BD".to_string(),
        image_index: 18,
        id: "ab".repeat(32),
        username: "arafat".to_string(),
        pc_name: "build".to_string(),
        tag: "phase-02".to_string(),
        encryption_key: "key".to_string(),
        signature: vec![9, 8, 7, 6],
        os: Os::Linux as i32,
        capabilities: 0b1011,
    };

    let raw = message.encode_to_vec();
    let frame = encode(&raw).expect("frame the protobuf bytes");

    assert_ne!(
        frame.as_slice(),
        raw.as_slice(),
        "the framed bytes must differ from the raw protobuf"
    );
    assert!(
        decode(&raw).is_err(),
        "raw protobuf is not a frame; the length prefix is required"
    );

    let payload = decode(&frame).expect("read the frame");
    assert_eq!(payload, raw.as_slice());

    let decoded = ClientIdentification::decode(payload).expect("decode the payload");
    assert_eq!(decoded, message);
}
