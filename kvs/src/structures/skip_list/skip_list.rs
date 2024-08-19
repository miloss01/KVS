use crate::Record;
use rand::Rng;
use std::{borrow::BorrowMut, cmp::Ordering};

#[derive(Clone, Debug)]
pub struct Node {
    record: Record,
    forward: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(record: Record, level: usize) -> Self {
        Node {
            record,
            forward: vec![None; level],
        }
    }
}

pub struct SkipList {
    pub head: Box<Node>,
    max_level: usize,
    height: usize,
    probability: f64,
}

impl SkipList {
    pub fn new(max_level: usize, probability: f64) -> Self {
        let head_record: Record = Record::new(vec![], vec![], false);
        SkipList {
            head: Box::new(Node::new(head_record, max_level)),
            max_level,
            height: 0,
            probability,
        }
    }

    fn random_level(&self) -> usize {
        let mut lvl: usize = 0;
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        while rng.gen_bool(self.probability) && lvl < self.max_level - 1 {
            lvl += 1;
        }
        lvl
    }

    pub fn insert(&mut self, record: &Record) {
        let new_level = self.random_level();
        let mut new_node = Node::new(record.clone(), self.max_level);

        for level in 0..self.max_level {
            let mut current_node = self.head.as_mut();
            let mut updated = false;

            while let Some(next) = current_node.forward[level].as_deref_mut() {
                match next.record.key.cmp(&record.key) {
                    Ordering::Less => {
                        current_node = current_node.forward[level].as_mut().unwrap();
                    }
                    Ordering::Equal => {
                        next.record = record.clone();
                        updated = true;
                        break;
                    }
                    Ordering::Greater => break,
                }
            }

            if updated {
                continue;
            }

            if level <= new_level {
                if current_node.forward[level].is_none() {
                    new_node.forward[level] = None;
                    current_node.forward[level] = Some(Box::new(new_node.clone()));
                } else {
                    let next = current_node.forward[level].as_deref().unwrap();
                    new_node.forward[level] = Some(Box::new(next.clone()));
                    current_node.forward[level] = Some(Box::new(new_node.clone()));
                }
            }
        }
    }

    pub fn search(&self, key: Vec<u8>) -> Option<&Record> {
        for level in (0..self.max_level).rev() {
            let mut node = &self.head;
            while let Some(ref next) = node.forward[level] {
                match next.record.key.cmp(&key) {
                    Ordering::Less => node = next,
                    Ordering::Equal => {
                        println!("na levelu {:?}", level);
                        return Some(&next.record);
                    }
                    Ordering::Greater => break,
                }
            }
        }
        None
    }

    pub fn get_all_records(&self) -> Vec<&Record> {
        let mut records = Vec::new();
        let mut node = &self.head;
        while let Some(ref next) = node.forward[0] {
            records.push(&next.record);
            node = next;
        }
        records
    }

    pub fn print(&self) {
        for level in 0..self.max_level {
            print!("Level {}: ", level);
            let mut node = &self.head;
            while let Some(ref next) = node.forward[level] {
                print!(
                    "({:?}, {:?}, {}), ",
                    next.record.key, next.record.value, next.record.tombstone
                );
                node = next;
            }
            println!();
        }
    }
}
