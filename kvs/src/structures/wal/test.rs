use crate::{Record, Wal};
use std::fs;
use std::path::Path;

#[test]
fn test_add_records() {
    let mut wal: Wal = Wal::new("test_data/wal1", 2, 2);

    let rec1: Record = Record::new("key1".to_string().into_bytes(), vec![1, 2, 3], false);
    let rec2: Record = Record::new("key2".to_string().into_bytes(), vec![1, 2, 3], false);
    let rec3: Record = Record::new("key3".to_string().into_bytes(), vec![1, 2, 3], false);
    let rec4: Record = Record::new("key4".to_string().into_bytes(), vec![1, 2, 3], false);
    let rec5: Record = Record::new("key5".to_string().into_bytes(), vec![1, 2, 3], false);

    wal.add_record(&rec1);
    wal.add_record(&rec2);

    // sad ima dva od dva, ima segment_1.wal i segment_2.wal

    let wal_check: Wal = Wal::new("test_data/wal1", 2, 2);
    let rec_check: Vec<Record> = vec![];
    assert_eq!(wal_check.current_records, rec_check);
    assert!(Path::new("test_data/wal1/segment_1.wal").exists());
    assert!(Path::new("test_data/wal1/segment_2.wal").exists());
    for i in 3..=10 {
        assert!(!Path::new(&format!("test_data/wal1/segment_{}.wal", i).to_string()).exists());
    }

    // sad se pravi segment_3.wal, pa se brise sve, jer je preslo max_segments
    wal.add_record(&rec3);
    wal.add_record(&rec4);
    assert!(!Path::new("test_data/wal1/segment_1.wal").exists());
    assert!(!Path::new("test_data/wal1/segment_2.wal").exists());
    assert!(!Path::new("test_data/wal1/segment_3.wal").exists());

    // sad se pravi segment_1.wal sa rec5
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
