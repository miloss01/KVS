use crate::{Record, Wal};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_add_records() {
    let mut wal: Wal = Wal::new("test_data/wal", 3, 3);

    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    wal.add_record(&(Record::new("key1".to_string(), vec![1, 2, 3], timestamp, false)));
    wal.add_record(&(Record::new("key2".to_string(), vec![1, 2, 3], timestamp, false)));
    wal.add_record(&(Record::new("key3".to_string(), vec![1, 2, 3], timestamp, true)));

    wal.add_record(&(Record::new("key4".to_string(), vec![1, 2, 3], timestamp, true)));

    let records: Vec<Record> = wal.current_records;
    println!("{:?}", records);
}
