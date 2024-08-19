use crate::{Record, SkipList};

#[test]
fn test_search() {
    let mut records: Vec<Record> = Vec::new();

    for i in 1..=10 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for _i in 1..100 {
        let mut skip_list: SkipList = SkipList::new(10, 0.5);

        for record in &records {
            skip_list.insert(record.clone());
        }

        for i in 0..10 {
            let record: &Record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
        }

        assert!(skip_list.search(vec![15 as u8]).is_none());
    }
}

#[test]
fn test_get_all_records() {
    let mut records: Vec<Record> = Vec::new();

    for i in 1..=10 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    let mut skip_list: SkipList = SkipList::new(10, 0.5);

    for record in &records {
        skip_list.insert(record.clone());
    }

    let got_records: Vec<&Record> = skip_list.get_all_records();
    let record_refs: Vec<&Record> = records.iter().collect();
    assert_eq!(record_refs, got_records);
}

#[test]
fn test_update() {
    let mut records: Vec<Record> = Vec::new();

    for i in 1..=10 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    for _i in 1..100 {
        let mut skip_list: SkipList = SkipList::new(10, 0.5);

        for record in &records {
            skip_list.insert(record.clone());
        }

        for i in 0..10 {
            let record: &Record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
            assert_eq!(record.tombstone, records[i].tombstone);
        }

        records[2].tombstone = true;
        records[5].tombstone = true;
        records[8].tombstone = true;

        skip_list.insert(records[2].clone());
        skip_list.insert(records[5].clone());
        skip_list.insert(records[8].clone());

        for i in 0..10 {
            let record: &Record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
            assert_eq!(record.tombstone, records[i].tombstone);
        }

        assert!(skip_list.search(vec![3]).unwrap().tombstone);
        assert!(skip_list.search(vec![6]).unwrap().tombstone);
        assert!(skip_list.search(vec![9]).unwrap().tombstone);
    }
}

#[test]
fn test_reset() {
    let mut records: Vec<Record> = Vec::new();

    for i in 1..=10 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        let record: Record = Record::new(key, value, false);
        records.push(record);
    }

    let mut skip_list: SkipList = SkipList::new(10, 0.5);

    for record in &records {
        skip_list.insert(record.clone());
    }

    for i in 0..10 {
        let record: &Record = skip_list.search(records[i].key.clone()).unwrap();
        assert_eq!(record.key, records[i].key);
        assert_eq!(record.tombstone, records[i].tombstone);
    }

    skip_list.reset();

    for i in 0..10 {
        let record: Option<&Record> = skip_list.search(records[i].key.clone());
        assert!(record.is_none());
    }

    for record in &records {
        skip_list.insert(record.clone());
    }

    for i in 0..10 {
        let record: &Record = skip_list.search(records[i].key.clone()).unwrap();
        assert_eq!(record.key, records[i].key);
        assert_eq!(record.tombstone, records[i].tombstone);
    }
}
