use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    capacity: u32,
    lookup: HashMap<Vec<u8>, Vec<u8>>,
    order: VecDeque<Vec<u8>>,
}

impl LRUCache {
    pub fn new(capacity: u32) -> Self {
        LRUCache {
            capacity,
            lookup: HashMap::new(),
            order: VecDeque::with_capacity(capacity as usize),
        }
    }

    pub fn get(&mut self, key: Vec<u8>) -> Option<&Vec<u8>> {
        if self.lookup.contains_key(&key) {
            self.update_order(&key);
        }
        self.lookup.get(&key)
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        if self.lookup.contains_key(&key) {
            self.update_order(&key);
        } else {
            if self.lookup.len() == self.capacity as usize {
                if let Some(lru_key) = self.order.pop_back() {
                    self.lookup.remove(&lru_key);
                }
            }
            self.order.push_front(key.clone());
        }

        self.lookup.insert(key, value);
    }

    pub fn remove(&mut self, key: Vec<u8>) {
        if self.lookup.contains_key(&key) {
            self.lookup.remove(&key);
            if let Some(pos) = self.order.iter().position(|x: &Vec<u8>| *x == key) {
                self.order.remove(pos);
            }
        }
    }

    pub fn get_order(&mut self) -> Vec<Vec<u8>> {
        self.order
            .iter()
            .rev()
            .map(|s: &Vec<u8>| s.clone())
            .collect()
    }

    fn update_order(&mut self, key: &Vec<u8>) {
        if let Some(pos) = self.order.iter().position(|x: &Vec<u8>| x == key) {
            self.order.remove(pos);
        }
        self.order.push_front(key.to_vec());
    }
}
