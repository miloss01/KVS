use crate::{Config, Record, KVS};

#[test]
fn proba() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    for i in 1..=15 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 11];
        kvs.put(key, value);
    }
}

#[test]
fn proba2() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);
    // let res = kvs.get(vec![13]);
    // println!("{:?}", res);
    // for i in 16..=20 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8 * 11];
    //     kvs.put(key, value);
    // }
    let res = kvs.get(vec![8]);
    println!("{:?}", res);
    let res = kvs.get(vec![8]);
    println!("{:?}", res);
    kvs.put(vec![102], vec![102]);
    let res = kvs.get(vec![102]);
    println!("{:?}", kvs.wal.current_records);
    println!("{:?}", kvs.memtable.data.get_all_records());
}

#[test]
fn proba3() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    let deleted = kvs.delete(vec![3]);
    let res = kvs.get(vec![56]);
    println!("{:?}", res);

    let deleted = kvs.delete(vec![102]);
    let res = kvs.get(vec![102]);
    println!("{:?}", res);
}

#[test]
fn proba4() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    // kvs.put(vec![102], vec![102]);
    // kvs.put(vec![102], vec![105]);

    let res = kvs.get(vec![102]);
    println!("{:?}", res);
}
