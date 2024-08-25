use crate::{Config, LRUCache, Memtable, Record, SSTable, TokenBucket, Wal};

pub struct KVS {
    config: Config,
    token_bucket: TokenBucket,
    cache: LRUCache,
    memtable: Memtable,
    wal: Wal,
}

impl KVS {
    pub fn new(config: &Config) -> Self {
        let wal: Wal = Wal::new(
            &config.wal_path,
            config.wal_max_segments,
            config.wal_max_elements,
        );
        let mut memtable: Memtable = Memtable::new(
            config.memtable_max_elements,
            config.skiplist_max_level as usize,
            config.skiplist_probability,
        );

        for record in &wal.current_records {
            memtable.insert(record.clone());
        }

        KVS {
            config: config.clone(),
            token_bucket: TokenBucket::new(
                config.token_bucket_max_elements,
                config.token_bucket_interval_in_secs,
            ),
            cache: LRUCache::new(config.cache_max_elements as u32),
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

        let mut sstable: SSTable = SSTable::new(
            &self.config.data_path,
            self.config.nth_element_in_summary,
            self.config.lsm_max_elements_per_level,
        );

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

            let mut sstable: SSTable = SSTable::new(
                &self.config.data_path,
                self.config.nth_element_in_summary,
                self.config.lsm_max_elements_per_level,
            );
            sstable.make(&flushed);
        }

        self.cache.put(key.clone(), record.serialize());
    }

    pub fn compact(&self) {
        let mut sstable: SSTable = SSTable::new(
            &self.config.data_path,
            self.config.nth_element_in_summary,
            self.config.lsm_max_elements_per_level,
        );

        sstable.compact();
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

            let mut sstable: SSTable = SSTable::new(
                &self.config.data_path,
                self.config.nth_element_in_summary,
                self.config.lsm_max_elements_per_level,
            );
            sstable.make(&flushed);
        }

        self.cache.remove(key.clone());

        true
    }
}
