use crate::{BloomFilter, MerkleTree, Record};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

pub struct SSTable {
    path: String,
    next_file_index: u64,
    nth_element_in_summary: u64,
}

impl SSTable {
    pub fn new(path: &str, nth_element_in_summary: u64) -> Self {
        SSTable {
            path: path.to_string(),
            next_file_index: 1,
            nth_element_in_summary,
        }
    }

    pub fn make(&mut self, records: &Vec<Record>) {
        self.next_file_index = self.get_next_index(1);

        self.make_bloom_filter_file(records);
        self.make_merkle_tree_file(records);

        let mut data_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!(
                "{}/data_1_{}.dat",
                self.path, self.next_file_index
            ))
            .unwrap();

        let mut index_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!(
                "{}/index_1_{}.dat",
                self.path, self.next_file_index
            ))
            .unwrap();

        let mut summary_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!(
                "{}/summary_1_{}.dat",
                self.path, self.next_file_index
            ))
            .unwrap();

        summary_file
            .write_all(&records[0].key_size.to_le_bytes())
            .unwrap();
        summary_file.write_all(&records[0].key).unwrap();
        summary_file
            .write_all(&records.last().unwrap().key_size.to_le_bytes())
            .unwrap();
        summary_file
            .write_all(&records.last().unwrap().key)
            .unwrap();

        let mut data_offset: u64 = 0;
        let mut index_offset: u64 = 0;
        let mut summary_counter: u64 = 0;

        for record in records {
            let serialized_record: Vec<u8> = record.serialize();

            data_file.write_all(&serialized_record).unwrap();

            index_file
                .write_all(&record.key_size.to_le_bytes())
                .unwrap();
            index_file.write_all(&record.key).unwrap();
            index_file.write_all(&data_offset.to_le_bytes()).unwrap();

            if summary_counter % self.nth_element_in_summary == 0
                || summary_counter == records.len() as u64 - 1
            {
                summary_file
                    .write_all(&record.key_size.to_le_bytes())
                    .unwrap();
                summary_file.write_all(&record.key).unwrap();
                summary_file.write_all(&index_offset.to_le_bytes()).unwrap();
            }

            summary_counter += 1;
            data_offset += serialized_record.len() as u64;
            index_offset += 8 + record.key.len() as u64 + 8;
        }
    }

    fn make_bloom_filter_file(&self, records: &Vec<Record>) {
        let bf_file_name: String = format!("{}/bf_1_{}.dat", self.path, self.next_file_index);

        let mut bloom_filter: BloomFilter = BloomFilter::new(records.len() as u32, 0.1);

        for record in records {
            bloom_filter.add(record.key.clone());
        }

        let _ = bloom_filter.save_to_file(&bf_file_name);
    }

    fn make_merkle_tree_file(&self, records: &Vec<Record>) {
        let mut keys: Vec<Vec<u8>> = Vec::new();

        for record in records {
            keys.push(record.key.clone());
        }

        let merkle_tree: MerkleTree = MerkleTree::new(&keys);

        merkle_tree.to_file(&format!("{}/mt_1_{}.dat", self.path, self.next_file_index));
    }

    pub fn search_sstable(&mut self, level: u64, index: u64, key: Vec<u8>) -> bool {
        let bloom_filter: BloomFilter =
            BloomFilter::load_from_file(&format!("{}/bf_{}_{}.dat", self.path, level, index))
                .unwrap();
        if !bloom_filter.contains(&key) {
            println!("nije prosao filter");
            return false;
        }

        let (first_key, last_key): (Vec<u8>, Vec<u8>) = self.get_summary_range(level, index);

        if &key < &first_key || &key > &last_key {
            println!("nije u summary range");
            return false;
        }

        return true;
    }

    fn get_summary_range(&self, level: u64, index: u64) -> (Vec<u8>, Vec<u8>) {
        let mut buffer: Vec<u8> = vec![0; 8];

        let mut summary_file: File = OpenOptions::new()
            .read(true)
            .open(&format!("{}/summary_{}_{}.dat", self.path, level, index))
            .unwrap();

        let mut first_key: Vec<u8> = Vec::new();
        let mut last_key: Vec<u8> = Vec::new();

        summary_file.read_exact(&mut buffer).unwrap();
        let key_size = u64::from_le_bytes(buffer[..8].try_into().unwrap());

        summary_file.seek(SeekFrom::Start(8)).unwrap();
        first_key.resize(key_size as usize, 0);
        summary_file.read_exact(&mut first_key).unwrap();

        summary_file
            .seek(SeekFrom::Start(8 + first_key.len() as u64))
            .unwrap();
        summary_file.read_exact(&mut buffer).unwrap();
        let key_size = u64::from_le_bytes(buffer[..8].try_into().unwrap());

        summary_file
            .seek(SeekFrom::Start(8 + first_key.len() as u64 + 8))
            .unwrap();
        last_key.resize(key_size as usize, 0);
        summary_file.read_exact(&mut last_key).unwrap();

        (first_key, last_key)
    }

    fn get_next_index(&self, level: u64) -> u64 {
        let paths: fs::ReadDir = fs::read_dir(self.path.clone()).unwrap();
        let search: String = format!("data_{}_", level);
        let mut index: u64 = 0;

        for path in paths {
            let mut file_name: String = path.unwrap().file_name().into_string().unwrap();
            if file_name.starts_with(&search) {
                file_name = file_name.replace(".dat", "");
                let tokens: Vec<&str> = file_name.split("_").collect();
                let num: u64 = tokens[2].parse().unwrap();

                if num > index {
                    index = num;
                }
            }
        }

        index + 1
    }
}
