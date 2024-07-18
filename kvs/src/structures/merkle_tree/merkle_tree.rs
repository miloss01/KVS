use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum PathItem {
    LEFT(Vec<u8>),
    RIGHT(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MerkleTree {
    nodes: Vec<Vec<Vec<u8>>>,
    pub leaves_count: u64,
}

impl MerkleTree {
    pub fn new(leaves: &Vec<Vec<u8>>) -> Self {
        let mut current_level: Vec<Vec<u8>> = leaves.iter().map(|leaf| Self::hash(&leaf)).collect();
        let mut nodes: Vec<Vec<Vec<u8>>> = Vec::new();

        while current_level.len() > 1 {
            if current_level.len() % 2 != 0 {
                current_level.push(current_level.last().unwrap().clone());
            }

            let mut next_level: Vec<Vec<u8>> = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                let combined: Vec<u8> = [&current_level[i][..], &current_level[i + 1][..]].concat();
                let hash: Vec<u8> = Self::hash(&combined);
                next_level.push(hash);
            }

            nodes.push(current_level);
            current_level = next_level;
        }

        nodes.push(current_level);

        MerkleTree {
            leaves_count: if leaves.len() == 1 {
                1
            } else if leaves.len() % 2 == 0 {
                leaves.len() as u64
            } else {
                (leaves.len() + 1) as u64
            },
            nodes,
        }
    }

    pub fn get_proof(&self, data: Vec<u8>) -> Vec<PathItem> {
        let hash: Vec<u8> = Self::hash(&data);
        let leaves: Vec<Vec<u8>> = self.nodes[0].clone();
        let mut path: Vec<PathItem> = Vec::new();

        if !leaves.contains(&hash) {
            return path;
        }

        let mut curr_idx: usize = leaves.iter().position(|x| *x == hash).unwrap();
        let curr_direction: PathItem = match curr_idx % 2 == 0 {
            true => PathItem::LEFT(data),
            false => PathItem::RIGHT(data),
        };
        path.push(curr_direction);

        for level in self.nodes.iter().take(self.nodes.len() - 1) {
            let sibling_direction: PathItem;

            match curr_idx % 2 == 0 {
                true => {
                    let sibling_idx: usize = curr_idx + 1;
                    sibling_direction = PathItem::RIGHT(level[sibling_idx].clone());
                }
                false => {
                    let sibling_idx: usize = curr_idx - 1;
                    sibling_direction = PathItem::LEFT(level[sibling_idx].clone());
                }
            };

            path.push(sibling_direction);
            curr_idx /= 2;
        }

        path
    }

    pub fn get_root_from_proof(proof: Vec<PathItem>) -> Vec<u8> {
        let mut curr: Vec<u8> = Vec::new();

        for path_item in &proof {
            match path_item {
                PathItem::LEFT(data) => curr = Self::hash(&[&data[..], &curr[..]].concat()),
                PathItem::RIGHT(data) => curr = Self::hash(&[&curr[..], &data[..]].concat()),
            }
        }

        curr
    }

    pub fn root(&self) -> Vec<u8> {
        self.nodes.last().unwrap()[0].clone()
    }

    pub fn to_file(&self, file_path: &str) {
        let json: String = serde_json::to_string(self).unwrap();
        let mut file: File = File::create(file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn from_file(file_path: &str) -> Self {
        let mut file: File = File::open(file_path).unwrap();
        let mut json: String = String::new();
        file.read_to_string(&mut json).unwrap();
        let merkle_tree: MerkleTree = serde_json::from_str(&json).unwrap();

        merkle_tree
    }

    fn hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
        // data.to_vec()
    }
}

//                 1
//       2                   3
//   4        5         6         7
// 8   9   10   11   12   13   14   15

// 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15

// leva deca: neparni indeksi, desna deca: parni ideksi
// levo dete: 2i + 1, desno dete: 2i + 2
// roditelj: if (child_idx % 2 == 0) then (child_idx / 2) - 1 else child_idx / 2 na manju
// sibling: if (child_idx % 2 == 0) then child_idx - 1 else child_idx + 1
