use crate::{structures::memtable, LRUCache, Memtable, Record, SSTable, TokenBucket, Wal};

pub struct KVS {
    token_bucket: TokenBucket,
    cache: LRUCache,
    memtable: Memtable,
    wal: Wal,
}

impl KVS {
    pub fn new() -> Self {
        let wal: Wal = Wal::new("data/wal", 5, 10);
        let mut memtable: Memtable = Memtable::new(10, 5, 0.5);

        for record in &wal.current_records {
            memtable.insert(record.clone());
        }

        KVS {
            token_bucket: TokenBucket::new(100, 3),
            cache: LRUCache::new(5),
            memtable,
            wal,
        }
    }

    pub fn get(&mut self, key: Vec<u8>) -> Option<Record> {
        let has_token: bool = self.token_bucket.use_token();

        if !has_token {
            println!("nema tokena");
            return None;
        }

        if let Some(record) = self.memtable.search(key.clone()) {
            println!("u memtable");
            return Some(record.clone());
        }

        if let Some(record) = self.cache.get(key.clone()) {
            println!("u cache");
            return Some(Record::deserialize(record));
        }

        let mut sstable: SSTable = SSTable::new("data", 2);

        if let Some(record) = sstable.search_all_sstables(key.clone()) {
            println!("u sstable");
            self.cache.put(key.clone(), record.serialize());
            return Some(record);
        }

        None
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let has_token: bool = self.token_bucket.use_token();

        if !has_token {
            println!("nema tokena");
            return;
        }

        let record: Record = Record::new(key.clone(), value, false);

        self.wal.add_record(&record);
        self.memtable.insert(record.clone());

        if self.memtable.is_full() {
            let flushed: Vec<Record> = self.memtable.flush();

            let mut sstable: SSTable = SSTable::new("data", 3);
            sstable.make(&flushed);
        }

        self.cache.put(key.clone(), record.serialize());
    }

    pub fn delete(&mut self, key: Vec<u8>) -> bool {
        let has_token: bool = self.token_bucket.use_token();

        if !has_token {
            println!("nema tokena");
            return false;
        }

        let res: Option<Record> = self.get(key.clone());

        if res.is_none() {
            return false;
        }

        let mut record: Record = res.unwrap();
        record.tombstone = true;

        self.wal.add_record(&record);
        self.memtable.insert(record.clone());

        if self.memtable.is_full() {
            let flushed: Vec<Record> = self.memtable.flush();

            let mut sstable: SSTable = SSTable::new("data", 3);
            sstable.make(&flushed);
        }

        self.cache.remove(key.clone());

        true
    }
}
