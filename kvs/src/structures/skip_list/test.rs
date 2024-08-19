use crate::{Record, SkipList};

#[test]
fn proba() {
    let mut skip_list: SkipList = SkipList::new(10, 0.5);

    let record1: Record = Record::new(vec![1], vec![11], false);
    let mut record2: Record = Record::new(vec![2], vec![22], false);
    let record3: Record = Record::new(vec![3], vec![33], false);

    skip_list.insert(&record1);
    skip_list.insert(&record2);
    skip_list.insert(&record3);

    println!("{:?}", skip_list.search(vec![1]));
    println!("{:?}", skip_list.search(vec![2]));
    println!("{:?}", skip_list.search(vec![3]));
    println!("{:?}", skip_list.search(vec![4]));
    println!("{:?}", skip_list.search(vec![5]));

    record2.key = vec![5];
    skip_list.insert(&record2);

    println!("{:?}", skip_list.head);
    skip_list.print();

    println!("{:?}", skip_list.search(vec![1]));
    println!("{:?}", skip_list.search(vec![2]));
    println!("{:?}", skip_list.search(vec![3]));
    println!("{:?}", skip_list.search(vec![4]));
    println!("{:?}", skip_list.search(vec![5]));

    println!("{:?}", skip_list.get_all_records());
}

#[test]
fn test_search() {
    let mut records = Vec::new();

    for i in 1..=10 {
        let key = vec![i as u8];
        let value = vec![i as u8 * 11];
        let record = Record::new(key, value, false);
        records.push(record);
    }

    for _i in 1..100 {
        let mut skip_list: SkipList = SkipList::new(10, 0.5);

        for record in &records {
            skip_list.insert(record);
        }

        for i in 1..10 {
            let record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
        }
    }
}

#[test]
fn probaaa() {
    let mut skip_list: SkipList = SkipList::new(5, 0.5);
    // let mut records = Vec::new();

    for i in 1..=10 {
        let key = vec![i as u8];
        let value = vec![i as u8 * 11];
        let record = Record::new(key, value, false);
        // records.push(record);
        skip_list.insert(&record);
    }
    skip_list.insert(&Record::new(vec![1 as u8], vec![23 as u8], true));
    skip_list.insert(&Record::new(vec![5 as u8], vec![75 as u8], false));
    skip_list.insert(&Record::new(vec![11 as u8], vec![77 as u8], true));
    // skip_list.insert(&records[0]);

    skip_list.print();

    println!("{:?}", skip_list.search(vec![1 as u8]));
    println!("{:?}", skip_list.search(vec![5 as u8]));
    println!("{:?}", skip_list.search(vec![11 as u8]));
    println!("{:?}", skip_list.search(vec![15 as u8]));

    // println!("{:?}", skip_list.get_all_records());
}

#[test]
fn test_update() {
    let mut records = Vec::new();

    for i in 1..=10 {
        let key = vec![i as u8];
        let value = vec![i as u8 * 11];
        let record = Record::new(key, value, false);
        records.push(record);
    }

    for _i in 1..100 {
        let mut skip_list: SkipList = SkipList::new(10, 0.5);

        for record in &records {
            skip_list.insert(record);
        }

        for i in 1..10 {
            let record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
            assert_eq!(record.tombstone, records[i].tombstone);
        }

        records[2].tombstone = true;
        records[5].tombstone = true;
        records[8].tombstone = true;

        skip_list.insert(&records[2]);
        skip_list.insert(&records[5]);
        skip_list.insert(&records[8]);

        for i in 1..10 {
            let record = skip_list.search(records[i].key.clone()).unwrap();
            assert_eq!(record.key, records[i].key);
            assert_eq!(record.tombstone, records[i].tombstone);
        }

        assert!(skip_list.search(vec![3]).unwrap().tombstone);
        assert!(skip_list.search(vec![6]).unwrap().tombstone);
        assert!(skip_list.search(vec![9]).unwrap().tombstone);
    }
}
