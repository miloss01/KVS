use crate::{Record, SkipList};

pub struct Memtable {
    data: SkipList,
    max_elements: u64,
    current_elements_count: u64,
}

impl Memtable {
    pub fn new(max_elements: u64, skip_list_max_level: usize, skip_list_probability: f64) -> Self {
        Memtable {
            data: SkipList::new(skip_list_max_level, skip_list_probability),
            max_elements,
            current_elements_count: 0,
        }
    }

    pub fn insert(&mut self, record: Record) {
        if self.is_full() {
            return;
        }

        self.data.insert(record.clone());
        self.current_elements_count += 1;
    }

    pub fn search(&self, key: Vec<u8>) -> Option<&Record> {
        self.data.search(key)
    }

    pub fn is_full(&self) -> bool {
        self.current_elements_count == self.max_elements
    }

    pub fn delete(&mut self, key: Vec<u8>) -> bool {
        let record: Option<&Record> = self.search(key);

        if record.is_none() {
            return false;
        }

        let mut record: Record = record.unwrap().clone();
        record.tombstone = true;

        self.data.insert(record.clone());

        true
    }

    pub fn flush(&mut self) -> Vec<Record> {
        let mut records: Vec<Record> = Vec::new();
        for record in self.data.get_all_records() {
            records.push(record.clone());
        }
        self.current_elements_count = 0;
        self.data.reset();
        records
    }
}
