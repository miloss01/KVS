use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::f64;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};

pub struct BloomFilter {
    expected_elements: u32,
    false_positive_rate: f64,
    base_seed: u32,
    m: u32,
    k: u32,
    bits: Vec<bool>,
    hashers: Vec<DefaultHasher>,
}

impl BloomFilter {
    pub fn new(expected_elements: u32, false_positive_rate: f64) -> Self {
        let m: u32 = Self::calculate_m(expected_elements, false_positive_rate);
        let k: u32 = Self::calculate_k(expected_elements, m);
        let base_seed: u32 = rand::thread_rng().gen_range(1..=100_000_000) as u32;
        let hashers: Vec<DefaultHasher> = Self::generate_hashers(k, base_seed);

        BloomFilter {
            expected_elements,
            false_positive_rate,
            base_seed,
            m,
            k,
            bits: vec![false; m as usize],
            hashers,
        }
    }

    pub fn add<T: Hash>(&mut self, item: T) {
        for i in 0..self.k {
            let mut hasher: DefaultHasher = self.hashers[i as usize].clone();
            item.hash(&mut hasher);
            let hash: u64 = hasher.finish();
            let index: u64 = hash % (self.m as u64);
            self.bits[index as usize] = true;
        }
    }

    pub fn contains<T: Hash>(&self, item: T) -> bool {
        for i in 0..self.k {
            let mut hasher: DefaultHasher = self.hashers[i as usize].clone();
            item.hash(&mut hasher);
            let hash: u64 = hasher.finish();
            let index: u64 = hash % (self.m as u64);
            if !self.bits[index as usize] {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut buffer: Vec<u8> = Vec::new();

        buffer.extend(&self.expected_elements.to_le_bytes());
        buffer.extend(&self.false_positive_rate.to_le_bytes());
        buffer.extend(&self.base_seed.to_le_bytes());
        buffer.extend(&self.m.to_le_bytes());
        buffer.extend(&self.k.to_le_bytes());

        for bit in &self.bits {
            buffer.push(*bit as u8);
        }

        file.write_all(&buffer)?;

        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> std::io::Result<Self> {
        let mut file: File = OpenOptions::new().read(true).open(file_path)?;

        let mut buffer: [u8; 24] = [0; 4 + 8 + 4 + 4 + 4];
        file.read_exact(&mut buffer)?;

        let expected_elements: u32 = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as u32;
        let false_positive_rate: f64 = f64::from_le_bytes(buffer[4..12].try_into().unwrap()) as f64;
        let base_seed: u32 = u32::from_le_bytes(buffer[12..16].try_into().unwrap()) as u32;
        let m: u32 = u32::from_le_bytes(buffer[16..20].try_into().unwrap()) as u32;
        let k: u32 = u32::from_le_bytes(buffer[20..24].try_into().unwrap()) as u32;

        let mut bits: Vec<bool> = vec![false; m as usize];
        for bit in &mut bits {
            let mut byte: [u8; 1] = [0; 1];
            file.read_exact(&mut byte)?;
            *bit = byte[0] != 0;
        }

        let hashers: Vec<DefaultHasher> = Self::generate_hashers(k, base_seed);

        Ok(Self {
            expected_elements,
            false_positive_rate,
            base_seed,
            m,
            k,
            hashers,
            bits,
        })
    }

    fn calculate_m(expected_elements: u32, false_positive_rate: f64) -> u32 {
        ((expected_elements as f64 * false_positive_rate.ln().abs()) / (f64::consts::LN_2.powi(2)))
            .ceil() as u32
    }

    fn calculate_k(expected_elements: u32, m: u32) -> u32 {
        ((m as f64 / expected_elements as f64) * f64::consts::LN_2).ceil() as u32
    }

    fn generate_hashers(k: u32, base_seed: u32) -> Vec<DefaultHasher> {
        (0..k)
            .map(|i| {
                let mut hasher: DefaultHasher = DefaultHasher::new();
                hasher.write_usize((base_seed + i) as usize);
                hasher
            })
            .collect()
    }
}
