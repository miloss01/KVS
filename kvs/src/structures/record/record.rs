use crc;
use std::cmp::PartialEq;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{u64, u8};

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub key_size: u64,
    pub value_size: u64,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: u64,
    pub tombstone: bool,
    pub crc: u64,
}

impl Record {
    pub fn new(key: Vec<u8>, value: Vec<u8>, tombstone: bool) -> Self {
        let x25: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
        let crc: u64 = x25.checksum(&value);
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        Record {
            key_size: key.len() as u64,
            value_size: value.len() as u64,
            key,
            value,
            timestamp,
            tombstone,
            crc,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.extend(self.crc.to_le_bytes());
        buffer.extend(self.timestamp.to_le_bytes());
        buffer.extend((self.tombstone as u8).to_le_bytes());
        buffer.extend(self.key_size.to_le_bytes());
        buffer.extend(self.value_size.to_le_bytes());

        buffer.extend(&self.key);
        buffer.extend(&self.value);

        buffer
    }

    pub fn deserialize(buffer: &Vec<u8>) -> Self {
        let buffer: &[u8] = &buffer;

        let key_size: u64 = u64::from_le_bytes(buffer[17..25].try_into().unwrap());
        let value_size: u64 = u64::from_le_bytes(buffer[25..33].try_into().unwrap());

        Record {
            crc: u64::from_le_bytes(buffer[0..8].try_into().unwrap()),
            timestamp: u64::from_le_bytes(buffer[8..16].try_into().unwrap()),
            tombstone: u8::from_le_bytes(buffer[16..17].try_into().unwrap()) != 0,
            key_size,
            value_size,
            key: buffer[33..33 + key_size as usize].to_vec(),
            value: buffer[33 + key_size as usize..33 + key_size as usize + value_size as usize]
                .to_vec(),
        }
    }

    pub fn check_crc(&self) -> bool {
        let x25: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
        let crc: u64 = x25.checksum(&self.value);

        self.crc == crc
    }
}
