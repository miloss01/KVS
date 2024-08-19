use crate::{Memtable, Record};
use std::vec;

#[test]
fn test_search() {
    let mut memtable: Memtable = Memtable::new(5, 5, 0.5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=5 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    for i in 0..5 {
        let record: &Record = memtable.search(records[i].key.clone()).unwrap();
        assert_eq!(record.key, records[i].key);
        assert_eq!(record.tombstone, records[i].tombstone);
    }

    assert_eq!(memtable.search(vec![5 as u8]).is_none(), false);
    assert!(memtable.search(vec![6 as u8]).is_none());
    assert!(memtable.search(vec![7 as u8]).is_none());
}

#[test]
fn test_is_full() {
    let mut memtable: Memtable = Memtable::new(5, 5, 0.5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=5 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    assert!(memtable.is_full());
}

#[test]
fn test_insert() {
    let mut memtable: Memtable = Memtable::new(5, 5, 0.5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=5 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    for i in 0..5 {
        let record: &Record = memtable.search(records[i].key.clone()).unwrap();
        assert_eq!(record.key, records[i].key);
        assert_eq!(record.tombstone, records[i].tombstone);
    }

    memtable.insert(Record::new(vec![10 as u8], vec![100 as u8], false));
    memtable.insert(Record::new(vec![11 as u8], vec![111 as u8], false));
    memtable.insert(Record::new(vec![12 as u8], vec![112 as u8], false));

    assert_eq!(memtable.search(vec![10 as u8]).is_none(), true);
    assert_eq!(memtable.search(vec![11 as u8]).is_none(), true);
    assert_eq!(memtable.search(vec![12 as u8]).is_none(), true);
}

#[test]
fn test_flush() {
    let mut memtable: Memtable = Memtable::new(5, 5, 0.5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=5 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    let flushed: Vec<Record> = memtable.flush();

    assert_eq!(records, flushed);

    for i in 0..5 {
        assert_eq!(memtable.search(records[i].key.clone()).is_none(), true);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    for i in 0..5 {
        let record: &Record = memtable.search(records[i].key.clone()).unwrap();
        assert_eq!(record.key, records[i].key);
        assert_eq!(record.tombstone, records[i].tombstone);
    }
}

#[test]
fn test_delete() {
    let mut memtable: Memtable = Memtable::new(5, 5, 0.5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=5 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for record in &records {
        memtable.insert(record.clone());
    }

    assert_eq!(memtable.delete(vec![1 as u8]), true);
    assert_eq!(memtable.delete(vec![2 as u8]), true);
    assert_eq!(memtable.delete(vec![10 as u8]), false);

    assert_eq!(memtable.search(vec![1 as u8]).unwrap().tombstone, true);
    assert_eq!(memtable.search(vec![2 as u8]).unwrap().tombstone, true);
}
