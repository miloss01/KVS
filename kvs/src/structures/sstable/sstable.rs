use crate::{BloomFilter, MerkleTree, Record};
use std::{
    fs::{self, remove_file, rename, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

pub struct SSTable {
    path: String,
    next_file_index: u64,
    nth_element_in_summary: u64,
    lsm_max_elements_per_level: u64,
}

impl SSTable {
    pub fn new(path: &str, nth_element_in_summary: u64, lsm_max_elements_per_level: u64) -> Self {
        SSTable {
            path: path.to_string(),
            next_file_index: 1,
            nth_element_in_summary,
            lsm_max_elements_per_level,
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

    pub fn search_sstable(&mut self, level: u64, index: u64, key: Vec<u8>) -> Option<Record> {
        let bloom_filter: BloomFilter =
            BloomFilter::load_from_file(&format!("{}/bf_{}_{}.dat", self.path, level, index))
                .unwrap();
        if !bloom_filter.contains(&key) {
            println!("nije prosao filter");
            return None;
        }

        let (first_key, last_key): (Vec<u8>, Vec<u8>) = self.get_summary_range(level, index);

        if &key < &first_key || &key > &last_key {
            println!("nije u summary range");
            return None;
        }

        let index_offset: u64 = self.get_index_offset_from_summary(level, index, &key);

        let data_offset: u64 = self.get_data_offset_from_index(level, index, &key, index_offset);

        let data: Vec<u8> = self.get_data(level, index, data_offset);
        let record: Record = Record::deserialize(&data);

        Some(record)
    }

    pub fn search_all_sstables(&mut self, key: Vec<u8>) -> Option<Record> {
        let mut level_index_pairs: Vec<(u64, u64)> = Vec::new();
        let paths: fs::ReadDir = fs::read_dir(self.path.clone()).unwrap();

        for path in paths {
            let mut file_name: String = path.unwrap().file_name().into_string().unwrap();
            if file_name.starts_with("data_") {
                file_name = file_name.replace(".dat", "");
                let tokens: Vec<&str> = file_name.split("_").collect();
                let level: u64 = tokens[1].parse().unwrap();
                let index: u64 = tokens[2].parse().unwrap();
                level_index_pairs.push((level, index));
            }
        }

        level_index_pairs.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

        for (level, index) in level_index_pairs {
            if let Some(record) = self.search_sstable(level, index, key.clone()) {
                println!("nasao u {:?}, {:?}", level, index);
                return Some(record);
            }
        }

        None
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
        let key_size: u64 = u64::from_le_bytes(buffer[..8].try_into().unwrap());

        summary_file.seek(SeekFrom::Start(8)).unwrap();
        first_key.resize(key_size as usize, 0);
        summary_file.read_exact(&mut first_key).unwrap();

        summary_file
            .seek(SeekFrom::Start(8 + first_key.len() as u64))
            .unwrap();
        summary_file.read_exact(&mut buffer).unwrap();
        let key_size: u64 = u64::from_le_bytes(buffer[..8].try_into().unwrap());

        summary_file
            .seek(SeekFrom::Start(8 + first_key.len() as u64 + 8))
            .unwrap();
        last_key.resize(key_size as usize, 0);
        summary_file.read_exact(&mut last_key).unwrap();

        (first_key, last_key)
    }

    fn get_index_offset_from_summary(&self, level: u64, index: u64, key: &Vec<u8>) -> u64 {
        let mut summary_file: File = OpenOptions::new()
            .read(true)
            .open(&format!("{}/summary_{}_{}.dat", self.path, level, index))
            .unwrap();

        let (first_key, last_key): (Vec<u8>, Vec<u8>) = self.get_summary_range(level, index);

        let mut offset: u64 = 8 + first_key.len() as u64 + 8 + last_key.len() as u64;
        let mut key_size: u64;
        let mut buffer: Vec<u8> = vec![0; 8];
        let mut current_key: Vec<u8> = Vec::new();
        let mut next_key: Vec<u8> = Vec::new();
        let mut current_key_offset: u64;
        let mut next_key_offset: u64;
        let mut index_offset: u64 = 0;

        loop {
            summary_file.seek(SeekFrom::Start(offset)).unwrap();
            let mut bytes_read: usize = summary_file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                println!("iskocio na prvom");
                break;
            }

            key_size = u64::from_le_bytes(buffer[..8].try_into().unwrap());
            current_key.resize(key_size as usize, 0);
            summary_file.seek(SeekFrom::Start(offset + 8)).unwrap();
            summary_file.read_exact(&mut current_key).unwrap();
            summary_file
                .seek(SeekFrom::Start(offset + 8 + current_key.len() as u64))
                .unwrap();
            summary_file.read(&mut buffer).unwrap();
            current_key_offset = u64::from_le_bytes(buffer[..8].try_into().unwrap());

            summary_file
                .seek(SeekFrom::Start(offset + 8 + current_key.len() as u64 + 8))
                .unwrap();
            bytes_read = summary_file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                println!("iskocio na drugom");
                break;
            }

            key_size = u64::from_le_bytes(buffer[..8].try_into().unwrap());
            next_key.resize(key_size as usize, 0);
            summary_file
                .seek(SeekFrom::Start(
                    offset + 8 + current_key.len() as u64 + 8 + 8,
                ))
                .unwrap();
            summary_file.read_exact(&mut next_key).unwrap();
            summary_file
                .seek(SeekFrom::Start(
                    offset + 8 + current_key.len() as u64 + 8 + 8 + next_key.len() as u64,
                ))
                .unwrap();
            summary_file.read(&mut buffer).unwrap();
            next_key_offset = u64::from_le_bytes(buffer[..8].try_into().unwrap());

            println!("{:?} - {:?}", current_key, next_key);
            if key == &current_key || key > &current_key && key < &next_key {
                index_offset = current_key_offset;
                break;
            }

            if key == &next_key {
                index_offset = next_key_offset;
                break;
            }

            offset += 8 + current_key.len() as u64 + 8;
        }

        index_offset
    }

    fn get_data_offset_from_index(
        &self,
        level: u64,
        index: u64,
        key: &Vec<u8>,
        initial_offset: u64,
    ) -> u64 {
        let mut index_file: File = OpenOptions::new()
            .read(true)
            .open(&format!("{}/index_{}_{}.dat", self.path, level, index))
            .unwrap();

        let mut buffer: Vec<u8> = vec![0; 8];
        let mut data_offset: u64 = 0;
        let mut current_key: Vec<u8> = Vec::new();
        let mut offset: u64 = initial_offset;

        loop {
            index_file.seek(SeekFrom::Start(offset)).unwrap();

            let bytes_read: usize = index_file.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                println!("zavrsio index");
                break;
            }

            let key_size: u64 = u64::from_le_bytes(buffer[..8].try_into().unwrap());
            current_key.resize(key_size as usize, 0);
            index_file.seek(SeekFrom::Start(offset + 8)).unwrap();
            index_file.read_exact(&mut current_key).unwrap();

            if key == &current_key {
                index_file
                    .seek(SeekFrom::Start(offset + 8 + current_key.len() as u64))
                    .unwrap();
                index_file.read(&mut buffer).unwrap();
                data_offset = u64::from_le_bytes(buffer[..8].try_into().unwrap());

                return data_offset;
            }

            offset += 8 + current_key.len() as u64 + 8;
        }

        data_offset
    }

    fn get_data(&self, level: u64, index: u64, offset: u64) -> Vec<u8> {
        let mut data_file: File = OpenOptions::new()
            .read(true)
            .open(&format!("{}/data_{}_{}.dat", self.path, level, index))
            .unwrap();

        data_file.seek(SeekFrom::Start(offset)).unwrap();

        let mut buffer: Vec<u8> = vec![0; 33];
        let mut key: Vec<u8> = Vec::new();
        let mut value: Vec<u8> = Vec::new();

        data_file.read_exact(&mut buffer).unwrap();

        let key_size: u64 = u64::from_le_bytes(buffer[17..25].try_into().unwrap());
        let value_size: u64 = u64::from_le_bytes(buffer[25..33].try_into().unwrap());

        key.resize(key_size as usize, 0);
        value.resize(value_size as usize, 0);

        data_file.seek(SeekFrom::Start(offset + 33)).unwrap();
        data_file.read_exact(&mut key).unwrap();
        data_file
            .seek(SeekFrom::Start(offset + 33 + key_size))
            .unwrap();
        data_file.read_exact(&mut value).unwrap();

        [buffer, key, value].concat()
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

    fn get_next_level(&self) -> u64 {
        let paths: fs::ReadDir = fs::read_dir(self.path.clone()).unwrap();
        let search: String = format!("data_");
        let mut level: u64 = 0;

        for path in paths {
            let mut file_name: String = path.unwrap().file_name().into_string().unwrap();
            if file_name.starts_with(&search) {
                file_name = file_name.replace(".dat", "");
                let tokens: Vec<&str> = file_name.split("_").collect();
                let num: u64 = tokens[1].parse().unwrap();

                if num > level {
                    level = num;
                }
            }
        }

        level + 1
    }

    pub fn compact(&mut self) {
        let current_level: u64 = self.get_next_level() - 1;
        println!("{:?}", current_level);

        if current_level == 0 {
            return;
        }

        for i in 1..=current_level {
            if self.should_compact(i) {
                self.compact_level(i);
            }
        }
    }

    fn should_compact(&self, level: u64) -> bool {
        self.get_next_index(level) - 1 >= self.lsm_max_elements_per_level
    }

    fn compact_level(&mut self, level: u64) {
        File::create(&format!("{}/temp.dat", self.path)).unwrap();

        for file_index in 1..=self.lsm_max_elements_per_level {
            let mut temp_offset: u64 = 0;
            let mut curr_file_offset: u64 = 0;

            let mut new_file: File = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&format!("{}/new_file.dat", self.path))
                .unwrap();

            let mut temp_file: File = OpenOptions::new()
                .read(true)
                .open(&format!("{}/temp.dat", self.path))
                .unwrap();

            let mut curr_file: File = OpenOptions::new()
                .read(true)
                .open(&format!("{}/data_{}_{}.dat", self.path, level, file_index))
                .unwrap();

            loop {
                let temp_record_serialized: Vec<u8> =
                    self.get_next_record_serialized(&mut temp_file, temp_offset);
                let curr_record_serialized: Vec<u8> =
                    self.get_next_record_serialized(&mut curr_file, curr_file_offset);

                if temp_record_serialized.len() == 0 && curr_record_serialized.len() == 0 {
                    break;
                }

                if temp_record_serialized.len() == 0 {
                    new_file.write_all(&curr_record_serialized).unwrap();
                    curr_file_offset += curr_record_serialized.len() as u64;
                } else if curr_record_serialized.len() == 0 {
                    new_file.write_all(&temp_record_serialized).unwrap();
                    temp_offset += temp_record_serialized.len() as u64;
                } else {
                    let temp_record: Record = Record::deserialize(&temp_record_serialized);
                    let curr_record: Record = Record::deserialize(&curr_record_serialized);

                    if &temp_record.key < &curr_record.key {
                        new_file.write_all(&temp_record_serialized).unwrap();
                        temp_offset += temp_record_serialized.len() as u64;
                    } else if &curr_record.key < &temp_record.key {
                        new_file.write_all(&curr_record_serialized).unwrap();
                        curr_file_offset += curr_record_serialized.len() as u64;
                    } else {
                        if temp_record.timestamp > curr_record.timestamp {
                            if !temp_record.tombstone {
                                new_file.write_all(&temp_record_serialized).unwrap();
                            }
                        } else {
                            if !curr_record.tombstone {
                                new_file.write_all(&curr_record_serialized).unwrap();
                            }
                        }

                        temp_offset += temp_record_serialized.len() as u64;
                        curr_file_offset += curr_record_serialized.len() as u64;
                    }
                }
            }

            remove_file(format!("{}/temp.dat", self.path)).unwrap();
            rename(
                format!("{}/new_file.dat", self.path),
                format!("{}/temp.dat", self.path),
            )
            .unwrap();
            remove_file(format!("{}/data_{}_{}.dat", self.path, level, file_index)).unwrap();
            remove_file(format!("{}/index_{}_{}.dat", self.path, level, file_index)).unwrap();
            remove_file(format!(
                "{}/summary_{}_{}.dat",
                self.path, level, file_index
            ))
            .unwrap();
            remove_file(format!("{}/bf_{}_{}.dat", self.path, level, file_index)).unwrap();
            remove_file(format!("{}/mt_{}_{}.dat", self.path, level, file_index)).unwrap();
        }

        let new_index: u64 = self.get_next_index(level + 1);

        rename(
            format!("{}/temp.dat", self.path),
            format!("{}/data_{}_{}.dat", self.path, level + 1, new_index),
        )
        .unwrap();

        self.create_sstable_from_data_file(level + 1, new_index);
    }

    fn get_next_record_serialized(&self, file: &mut File, offset: u64) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 33];
        file.seek(SeekFrom::Start(offset)).unwrap();

        let bytes_read: usize = file.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            return Vec::new();
        }

        let key_size: u64 = u64::from_le_bytes(buffer[17..25].try_into().unwrap());
        let value_size: u64 = u64::from_le_bytes(buffer[25..33].try_into().unwrap());
        let mut key: Vec<u8> = vec![0; key_size as usize];
        let mut value: Vec<u8> = vec![0; value_size as usize];

        file.seek(SeekFrom::Start(offset + 33)).unwrap();
        file.read_exact(&mut key).unwrap();
        file.seek(SeekFrom::Start(offset + 33 + key_size)).unwrap();
        file.read_exact(&mut value).unwrap();

        [buffer, key, value].concat()
    }

    pub fn create_sstable_from_data_file(&self, level: u64, index: u64) {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut offsets: Vec<u64> = Vec::new();
        let mut data_offset: u64 = 0;

        let mut data_file: File = OpenOptions::new()
            .read(true)
            .open(&format!("{}/data_{}_{}.dat", self.path, level, index))
            .unwrap();

        loop {
            let record_serialized: Vec<u8> =
                self.get_next_record_serialized(&mut data_file, data_offset);

            if record_serialized.len() == 0 {
                break;
            }

            let record: Record = Record::deserialize(&record_serialized);

            keys.push(record.key.clone());
            offsets.push(data_offset);

            data_offset += record_serialized.len() as u64;
        }

        let mut index_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!("{}/index_{}_{}.dat", self.path, level, index))
            .unwrap();

        let mut summary_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!("{}/summary_{}_{}.dat", self.path, level, index))
            .unwrap();

        let mut bloom_filter: BloomFilter = BloomFilter::new(keys.len() as u32, 0.1);

        let merkle_tree: MerkleTree = MerkleTree::new(&keys);
        merkle_tree.to_file(&format!("{}/mt_{}_{}.dat", self.path, level, index));

        summary_file
            .write_all(&(keys[0].len() as u64).to_le_bytes())
            .unwrap();
        summary_file.write_all(&keys[0]).unwrap();
        summary_file
            .write_all(&(keys.last().unwrap().len() as u64).to_le_bytes())
            .unwrap();
        summary_file.write_all(&keys.last().unwrap()).unwrap();

        let mut index_offset: u64 = 0;
        let mut summary_counter: u64 = 0;

        for (i, key) in keys.iter().enumerate() {
            bloom_filter.add(key.clone());

            index_file
                .write_all(&(key.len() as u64).to_le_bytes())
                .unwrap();
            index_file.write_all(key).unwrap();
            index_file.write_all(&offsets[i].to_le_bytes()).unwrap();

            if summary_counter % self.nth_element_in_summary == 0
                || summary_counter == keys.len() as u64 - 1
            {
                summary_file
                    .write_all(&(key.len() as u64).to_le_bytes())
                    .unwrap();
                summary_file.write_all(key).unwrap();
                summary_file.write_all(&index_offset.to_le_bytes()).unwrap();
            }

            summary_counter += 1;
            index_offset += 8 + key.len() as u64 + 8;
        }

        bloom_filter
            .save_to_file(&format!("{}/bf_{}_{}.dat", self.path, level, index))
            .unwrap();
    }
}
