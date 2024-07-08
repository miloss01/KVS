use std::time::{SystemTime, UNIX_EPOCH};

use crate::Record;

#[test]
fn test_ser_deser() {
    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    let record: Record = Record::new("key1".to_string(), vec![1, 2, 3], timestamp, false);

    let serialized: Vec<u8> = record.serialize();
    let deserialized: Record = Record::deserialize(&serialized);

    assert_eq!(record.crc, deserialized.crc);
    assert_eq!(record.timestamp, deserialized.timestamp);
    assert_eq!(record.tombstone, deserialized.tombstone);
    assert_eq!(record.key_size, deserialized.key_size);
    assert_eq!(record.value_size, deserialized.value_size);
    assert_eq!(record.key, deserialized.key);
    assert_eq!(record.value, deserialized.value);

    assert!(record.check_crc());
    assert!(deserialized.check_crc());
}
