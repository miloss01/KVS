use crate::Record;
use rand::Rng;

#[derive(Clone, Debug)]
struct Node {
    record: Record,
    forward: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(record: Record, level: usize) -> Self {
        Node {
            record,
            forward: vec![None; level + 1],
        }
    }
}

pub struct SkipList {
    head: Box<Node>,
    max_level: usize,
    level: usize,
    probability: f64,
}

impl SkipList {
    pub fn new(max_level: usize, probability: f64) -> Self {
        let head_record: Record = Record::new(vec![], vec![], false);
        SkipList {
            head: Box::new(Node::new(head_record, max_level)),
            max_level,
            level: 0,
            probability,
        }
    }

    fn random_level(&self) -> usize {
        let mut lvl: usize = 0;
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        while rng.gen_bool(self.probability) && lvl < self.max_level {
            lvl += 1;
        }
        lvl
    }

    pub fn insert(&mut self, record: Record) {
        let new_level: usize = self.random_level();
        let new_node: Node = Node::new(record, new_level + 1);
    }
}
