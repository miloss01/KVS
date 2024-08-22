use crate::{Record, SSTable};

#[test]
fn proba() {
    let mut sstable: SSTable = SSTable::new("data", 2);

    // let mut records: Vec<Record> = Vec::new();

    // for i in 1..=5 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8 * 11];
    //     let record: Record = Record::new(key, value, false);
    //     records.push(record);
    // }

    // sstable.make(&records);

    for i in 1..=6 {
        let res = sstable.search_sstable(1, 1, vec![i as u8]);
        println!("{:?}", res);
    }
}
