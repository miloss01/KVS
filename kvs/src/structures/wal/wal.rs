use crate::Record;
use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct Wal {
    path: String,
    max_segments: u64,
    max_elements_per_segment: u64,
    current_segment: u64,
    current_element_count: u64,
    pub current_records: Vec<Record>,
}

impl Wal {
    pub fn new(path: &str, max_segments: u64, max_elements_per_segment: u64) -> Self {
        let mut current_segment: u64 = Self::find_latest_segment(path);
        let current_records: Vec<Record>;

        if current_segment == 0 {
            current_segment = 1;
            current_records = Vec::new();
        } else {
            current_records =
                Self::get_records_from_segment(format!("{}/segment_{}.wal", path, current_segment));
        }

        Wal {
            path: path.to_string(),
            max_segments,
            max_elements_per_segment,
            current_segment,
            current_element_count: current_records.len() as u64,
            current_records,
        }
    }

    pub fn add_record(&mut self, record: &Record) {
        if self.current_element_count >= self.max_elements_per_segment {
            self.current_segment += 1;
            self.current_element_count = 0;

            if self.current_segment > self.max_segments {
                self.cleanup_segments();
                self.current_segment = 1;
            }

            self.current_records.clear();
        }

        let segment_filename: String =
            format!("{}/segment_{}.wal", self.path, self.current_segment);
        let mut file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&segment_filename)
            .unwrap();

        let serialized_record: Vec<u8> = record.serialize();
        file.write_all(&serialized_record).unwrap();

        self.current_element_count += 1;
        self.current_records.push(record.clone());
    }

    fn cleanup_segments(&self) {
        for i in 1..=self.max_segments {
            let segment_filename: String = format!("{}/segment_{}.wal", self.path, i);
            if Path::new(&segment_filename).exists() {
                fs::remove_file(segment_filename).unwrap();
            }
        }
    }

    fn find_latest_segment(path: &str) -> u64 {
        let re: Regex = Regex::new(r"segment_(\d+)\.wal").unwrap();
        let mut max_segment: u64 = 0;
        let paths: fs::ReadDir = fs::read_dir(path).unwrap();

        for path in paths {
            let mut file_name: String = path.unwrap().file_name().into_string().unwrap();
            if re.is_match(&file_name) {
                file_name = file_name.replace(".wal", "");
                let tokens: Vec<&str> = file_name.split("_").collect();
                let num: u64 = tokens[1].parse().unwrap();

                if num > max_segment {
                    max_segment = num;
                }
            }
        }

        max_segment
    }

    fn get_records_from_segment(file_path: String) -> Vec<Record> {
        let mut file: File = OpenOptions::new().read(true).open(&file_path).unwrap();
        let mut start: usize = 0;
        let mut buffer: [u8; 33] = [0; 8 + 8 + 1 + 8 + 8];
        let mut records: Vec<Record> = Vec::new();

        loop {
            file.seek(SeekFrom::Start(start as u64)).unwrap();

            let bytes_read: usize = file.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                break;
            }

            let key_size: u64 = u64::from_le_bytes(buffer[17..25].try_into().unwrap());
            let value_size: u64 = u64::from_le_bytes(buffer[25..33].try_into().unwrap());

            let mut key_and_value: Vec<u8> = vec![0; key_size as usize + value_size as usize];
            file.read_exact(&mut key_and_value).unwrap();

            let mut data: Vec<u8> = Vec::from(buffer);
            data.append(&mut key_and_value);

            records.push(Record::deserialize(&data));

            start += data.len();
        }

        records
    }
}
