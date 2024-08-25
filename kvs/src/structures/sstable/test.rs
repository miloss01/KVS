use crate::{Record, SSTable};

#[test]
fn proba() {
    let mut sstable: SSTable = SSTable::new("data", 4, 5);

    let mut records: Vec<Record> = Vec::new();

    for i in 1..=250 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8];
        if i % 3 == 0 {
            let record: Record = Record::new(key, value, false);
            records.push(record);
        } else {
            let record: Record = Record::new(key, value, true);
            records.push(record);
        }
    }

    sstable.make(&records);

    for i in 0..250 {
        let res = sstable.search_all_sstables(vec![i as u8 + 1]);
        assert_eq!(res.unwrap(), records[i]);
    }
}

#[test]
fn proba2() {
    let mut sstable: SSTable = SSTable::new("data", 2, 5);
    // let mut records: Vec<Record> = Vec::new();
    // for i in 1..=5 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8];
    //     let record: Record = Record::new(key, value, false);
    //     records.push(record);
    // }
    // sstable.make(&records);
    // records.clear();
    // records.push(Record::new(vec![2], vec![33], true));
    // for i in 6..=12 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8];
    //     let record: Record = Record::new(key, value, false);
    //     records.push(record);
    // }
    // sstable.make(&records);

    for i in 3..=12 {
        let res = sstable.search_all_sstables(vec![i as u8]);
        assert_eq!(res.clone().unwrap().key, vec![i as u8]);
        assert_eq!(res.clone().unwrap().tombstone, false);
    }
    let res = sstable.search_all_sstables(vec![2]);
    assert_eq!(res.clone().unwrap().tombstone, true);
    assert_eq!(res.clone().unwrap().value, vec![33]);
}

#[test]
fn proba3() {
    let mut sstable: SSTable = SSTable::new("data", 2, 5);

    // let mut records: Vec<Record> = Vec::new();
    // for i in 1..=5 {
    //     let key: Vec<u8> = format!("key {:?}", i).into_bytes();
    //     let value: Vec<u8> = vec![i as u8];
    //     let record: Record = Record::new(key, value, false);
    //     records.push(record);
    // }
    // records.sort_by_key(|record| record.key.clone());
    // sstable.make(&records);
    // records.clear();
    // records.push(Record::new(
    //     "key 2".to_string().into_bytes(),
    //     vec![33],
    //     true,
    // ));
    // for i in 6..=12 {
    //     let key: Vec<u8> = format!("key {:?}", i).into_bytes();
    //     let value: Vec<u8> = vec![i as u8];
    //     let record: Record = Record::new(key, value, false);
    //     records.push(record);
    // }
    // records.sort_by_key(|record| record.key.clone());
    // sstable.make(&records);

    for i in 3..=12 {
        let res = sstable.search_all_sstables(format!("key {:?}", i).into_bytes());
        println!("res {:?}", res);
        assert_eq!(
            res.clone().unwrap().key,
            format!("key {:?}", i).into_bytes()
        );
        assert_eq!(res.clone().unwrap().tombstone, false);
    }
    let res = sstable.search_all_sstables("key 2".to_string().into_bytes());
    println!("res {:?}", res);
    assert_eq!(res.clone().unwrap().tombstone, true);
    assert_eq!(res.clone().unwrap().value, vec![33]);
}

#[test]
fn prrr() {
    let mut sstable: SSTable = SSTable::new("data", 2, 3);
    sstable.compact();
    // sstable.create_sstable_from_data_file(1, 1);
}
