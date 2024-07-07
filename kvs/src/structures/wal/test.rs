use crate::{Record, Wal};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_add_records() {
    let mut wal: Wal = Wal::new("test_data/wal1", 2, 2);

    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    let rec1: Record = Record::new("key1".to_string(), vec![1, 2, 3], timestamp, false);
    let rec2: Record = Record::new("key2".to_string(), vec![1, 2, 3], timestamp, false);
    let rec3: Record = Record::new("key3".to_string(), vec![1, 2, 3], timestamp, false);
    let rec4: Record = Record::new("key4".to_string(), vec![1, 2, 3], timestamp, false);
    let rec5: Record = Record::new("key5".to_string(), vec![1, 2, 3], timestamp, false);

    wal.add_record(&rec1);
    wal.add_record(&rec2);

    // sad ima dva od dva, samo segment_1.wal

    let wal_check: Wal = Wal::new("test_data/wal1", 2, 2);
    let rec_check: Vec<Record> = vec![rec1.clone(), rec2.clone()];
    assert_eq!(wal_check.current_records, rec_check);
    assert!(Path::new("test_data/wal1/segment_1.wal").exists());
    for i in 2..=10 {
        assert!(!Path::new(&format!("test_data/wal1/segment_{}.wal", i).to_string()).exists());
    }

    // sad se pravi segment_2.wal
    wal.add_record(&rec3);
    assert!(Path::new("test_data/wal1/segment_2.wal").exists());

    wal.add_record(&rec4);

    // sad treba da se izbrisu segment_1.wal i segment_2.wal pa se pravi segment1.wal sa &rec5
    wal.add_record(&rec5);

    let wal_check: Wal = Wal::new("test_data/wal1", 2, 2);
    let rec_check: Vec<Record> = vec![rec5.clone()];
    assert_eq!(wal_check.current_records, rec_check);
    assert!(Path::new("test_data/wal1/segment_1.wal").exists());
    for i in 2..=10 {
        assert!(!Path::new(&format!("test_data/wal1/segment_{}.wal", i).to_string()).exists());
    }

    fs::remove_dir_all("test_data/wal1").unwrap();
    fs::create_dir("test_data/wal1").unwrap();
}
