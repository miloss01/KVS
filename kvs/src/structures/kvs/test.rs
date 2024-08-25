use crate::{Config, Record, KVS};

#[test]
fn proba() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    for i in 11..=15 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![200];
        kvs.put(key, value);
        // kvs.delete(vec![i as u8]);
    }
}

#[test]
fn proba2() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);
    // let res = kvs.get(vec![13]);
    // println!("{:?}", res);
    for i in 16..=20 {
        let key: Vec<u8> = vec![i as u8];
        let value: Vec<u8> = vec![i as u8 * 12];
        kvs.put(key, value);
    }
    // let res = kvs.get(vec![8]);
    // println!("{:?}", res);
    // let res = kvs.get(vec![8]);
    // println!("{:?}", res);
    // kvs.put(vec![102], vec![102]);
    // let res = kvs.get(vec![102]);
    // println!("{:?}", kvs.wal.current_records);
    // println!("{:?}", kvs.memtable.data.get_all_records());
}

#[test]
fn proba3() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    // let deleted = kvs.delete(vec![3]);
    // let res = kvs.get(vec![56]);
    // println!("{:?}", res);

    // let deleted = kvs.delete(vec![102]);
    let res = kvs.get(vec![3]);
    println!("{:?}", res);
    let res = kvs.get(vec![102]);
    println!("{:?}", res);
}

#[test]
fn proba4() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    // kvs.put(vec![102], vec![102]);
    // kvs.put(vec![102], vec![105]);
    // kvs.delete(vec![1]);
    // kvs.delete(vec![4]);
    // kvs.delete(vec![6]);
    // println!("{:?}", kvs.memtable.data.get_all_records());
    // let res = kvs.get(vec![102]);
    // println!("{:?}", res);
    for i in 1..=20 {
        println!("trazi {:?}", i);
        let res = kvs.get(vec![i as u8]);
        println!("{:?}", res);
    }
}

#[test]
fn test_compact() {
    let config: Config = Config::new("data/config/config.json");
    let mut kvs: KVS = KVS::new(&config);

    // for i in 1..=15 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8];
    //     kvs.put(key, value);
    // }

    // for i in 1..=5 {
    //     kvs.delete(vec![i as u8]);
    // }

    // for i in 16..=20 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![i as u8];
    //     kvs.put(key, value);
    // }

    // for i in 11..=15 {
    //     let key: Vec<u8> = vec![i as u8];
    //     let value: Vec<u8> = vec![200];
    //     kvs.put(key, value);
    // }

    // kvs.compact();

    for i in 1..=5 {
        println!("trazi {:?}", i);
        assert_eq!(kvs.get(vec![i as u8]).is_none(), true);
    }

    for i in 6..=10 {
        println!("trazi {:?}", i);
        assert_eq!(kvs.get(vec![i as u8]).unwrap().value, vec![i as u8]);
    }

    for i in 11..=15 {
        println!("trazi {:?}", i);
        assert_eq!(kvs.get(vec![i as u8]).unwrap().value, vec![200]);
    }

    for i in 16..=20 {
        println!("trazi {:?}", i);
        assert_eq!(kvs.get(vec![i as u8]).unwrap().value, vec![i as u8]);
    }
}
