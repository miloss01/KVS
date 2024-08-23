use crate::{Record, KVS};

#[test]
fn proba() {
    let mut kvs: KVS = KVS::new();

    for i in 1..=15 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        kvs.put(key, value);
    }
}

#[test]
fn proba2() {
    let mut kvs: KVS = KVS::new();
    // let res = kvs.get(vec![13]);
    // println!("{:?}", res);
    // for i in 16..=20 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8 * 11];
    //     kvs.put(key, value);
    // }
    // let res = kvs.get(vec![8]);
    // println!("{:?}", res);
    // let res = kvs.get(vec![8]);
    // println!("{:?}", res);
    // kvs.put(vec![100], vec![100]);
    let res = kvs.get(vec![100]);
    println!("{:?}", res);
}

#[test]
fn proba3() {
    let mut kvs: KVS = KVS::new();

    // let deleted = kvs.delete(vec![3]);
    let res = kvs.get(vec![56]);
    println!("{:?}", res);

    // let deleted = kvs.delete(vec![100]);
    // let res = kvs.get(vec![100]);
    // println!("{:?}", res);
}
