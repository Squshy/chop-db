use super::rand::generate_random_string;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::sync::RwLock;

const DELETED_FLAG: usize = 0xFF;

pub struct Segment {
    /// A hash map which references a key and byte-offset location in the segment
    ///
    /// The offset is the starting location of the key's data. The first x bytes
    /// is the size of the data which is stored followed by the raw data.
    hash_map: RwLock<HashMap<String, usize>>,
    byte_offset: RwLock<usize>,
    file: RwLock<File>,
}

fn create_file(file_name: String) -> Result<String, anyhow::Error> {
    let path = "./logs/";
    let file_path = format!("{}/{}.chop", path, file_name);
    DirBuilder::new().recursive(true).create(&path)?;

    Ok(file_path.to_string())
}

#[repr(usize)]
pub enum LogEntry {
    Deleted = DELETED_FLAG,
    Alive(String),
}

impl Segment {
    pub fn new() -> Result<Self, anyhow::Error> {
        let file_path = create_file(generate_random_string(15))?;
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)?;

        let segment = Segment {
            hash_map: RwLock::new(HashMap::new()),
            byte_offset: RwLock::new(0),
            file: RwLock::new(file),
        };

        Ok(segment)
    }

    pub fn get(&self, key: &String) -> Result<Option<String>, anyhow::Error> {
        let offset = match self.hash_map.read().unwrap().get(key) {
            Some(&offset) => offset,
            None => return Ok(None),
        };

        let file = self.file.read().unwrap();
        let mut file = file.deref();

        // Read the file from our offset
        file.seek(SeekFrom::Start(offset as u64))?;
        // Read the size of the data which is stored
        let mut offset_buffer = [0u8; 8];
        file.read_exact(&mut offset_buffer)?;

        // Convert the bytes to an int in a funky way
        let data_size: usize =
            u64::from_le_bytes(offset_buffer.as_slice().try_into().unwrap()) as usize;

        // Read out the data stored starting from the offset + 8 (size of number
        // which tells us how big the data is)
        file.seek(SeekFrom::Start((offset + 8) as u64))?;
        let mut buffer = vec![0u8; data_size];
        file.read_exact(&mut buffer)?;

        if buffer.last() == Some(&(DELETED_FLAG as u8)) || buffer.last() == None {
            return Ok(None);
        }

        Ok(Some(String::from_utf8(buffer)?))
    }

    pub fn set(&self, key: &String, data: &LogEntry) -> Result<(), anyhow::Error> {
        let data_bytes: &[u8] = match data {
            LogEntry::Alive(d) => d.as_bytes(),
            LogEntry::Deleted => &[DELETED_FLAG as u8],
        };

        let data_byte_size = data_bytes.len().to_le_bytes();

        if data_bytes.len() > usize::MAX {
            return Err(anyhow!("Data must be at most 64/32 bytes large"));
        };

        let mut file = self.file.write().unwrap();
        file.write(&data_byte_size)?;
        file.write(data_bytes)?;

        self.hash_map
            .write()
            .unwrap()
            .entry(key.clone())
            .and_modify(|val| *val = *self.byte_offset.read().unwrap())
            .or_insert_with(|| *self.byte_offset.read().unwrap());

        *self.byte_offset.write().unwrap() += data_bytes.len() + 8;

        Ok(())
    }

    pub fn delete(&self, key: &String) -> Result<bool, anyhow::Error> {
        self.set(&key, &LogEntry::Deleted)?;

        Ok(true)
    }

    pub fn has_key(&self, key: &String) -> bool {
        self.hash_map.read().unwrap().contains_key(key)
    }
}
