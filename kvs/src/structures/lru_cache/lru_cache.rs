use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    capacity: usize,
    lookup: HashMap<String, Vec<u8>>,
    order: VecDeque<String>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            lookup: HashMap::new(),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
        if self.lookup.contains_key(key) {
            self.update_order(key.to_string());
        }
        self.lookup.get(key)
    }

    pub fn put(&mut self, key: &str, value: Vec<u8>) {
        if self.lookup.contains_key(key) {
            self.update_order(key.to_string());
        } else {
            if self.lookup.len() == self.capacity {
                if let Some(lru_key) = self.order.pop_back() {
                    self.lookup.remove(&lru_key);
                }
            }
            self.order.push_front(key.to_string());
        }

        self.lookup.insert(key.to_string(), value);
    }

    pub fn get_order(&mut self) -> Vec<&str> {
        self.order.iter().rev().map(|s| s.as_str()).collect()
    }

    fn update_order(&mut self, key: String) {
        if let Some(pos) = self.order.iter().position(|x: &String| *x == key) {
            self.order.remove(pos);
        }
        self.order.push_front(key);
    }
}

/*
    VecDeque
    front je levo (najskorije dodati), back je desno (najranije dodati)
*/
