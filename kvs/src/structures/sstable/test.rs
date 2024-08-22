use crate::{Record, SSTable};

#[test]
fn proba() {
    let mut sstable: SSTable = SSTable::new("data", 4);

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
        let res = sstable.search_sstable(1, 1, vec![i as u8 + 1]);
        assert_eq!(res.unwrap(), records[i]);
    }
}
