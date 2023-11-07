use anyhow::anyhow;
use hashy::rand::generate_random_string;
use std::collections::HashMap;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const DELETED_FLAG: usize = 0xFF;

struct Segment {
    /// A hash map which references a key and byte-offset location in the segment
    ///
    /// The offset is the starting location of the key's data. The first x bytes
    /// is the size of the data which is stored followed by the raw data.
    hash_map: HashMap<String, usize>,
    file_path: String,
    byte_offset: usize,
}

fn create_file(file_name: String) -> Result<String, anyhow::Error> {
    let path = "./logs/";
    let file_path = format!("{}/{}", path, file_name);
    DirBuilder::new().recursive(true).create(&path)?;

    Ok(file_path.to_string())
}

#[repr(usize)]
enum LogEntry {
    Deleted = DELETED_FLAG,
    Alive(String),
}

impl Segment {
    pub fn new() -> Result<Self, anyhow::Error> {
        let file_path = create_file(generate_random_string(15))?;
        let segment = Segment {
            hash_map: HashMap::new(),
            file_path,
            byte_offset: 0,
        };

        File::create(&segment.file_path)?;

        Ok(segment)
    }

    pub fn get(&self, key: &String) -> Result<Option<String>, anyhow::Error> {
        let offset = self.hash_map.get(key);

        if offset.is_none() {
            return Ok(None);
        }

        let offset = offset.unwrap();
        let mut file = File::open(&self.file_path)?;

        // Read the file from our offset
        file.seek(SeekFrom::Start(*offset as u64))?;
        // Read the size of the data which is stored
        let mut offset_buffer = [0u8; 8 as usize];
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

    pub fn set(&mut self, key: &String, data: &LogEntry) -> Result<(), anyhow::Error> {
        let mut file = OpenOptions::new().append(true).open(&self.file_path)?;

        let data_bytes: &[u8] = match data {
            LogEntry::Alive(d) => d.as_bytes(),
            LogEntry::Deleted => &[DELETED_FLAG as u8],
        };

        let data_byte_size = data_bytes.len().to_le_bytes();

        if data_bytes.len() > usize::MAX {
            return Err(anyhow!("Data must be at most 64/32 bytes large"));
        };

        file.write(&data_byte_size)?;
        let num_bytes = file.write(data_bytes)?;

        self.hash_map
            .entry(key.clone())
            .and_modify(|val| *val = self.byte_offset)
            .or_insert_with(|| self.byte_offset);

        self.byte_offset += num_bytes + 8;

        Ok(())
    }

    pub fn delete(&mut self, key: &String) -> Result<bool, anyhow::Error> {
        self.set(&key, &LogEntry::Deleted)?;

        Ok(true)
    }

    pub fn has_key(&self, key: &String) -> bool {
        self.hash_map.contains_key(key)
    }
}

struct HashIndex {
    segments: Vec<Segment>,
}

impl HashIndex {
    fn new() -> Result<Self, anyhow::Error> {
        let segment = Segment::new()?;
        let hash_index = HashIndex {
            segments: vec![segment],
        };

        Ok(hash_index)
    }

    fn get(&self, key: &String) -> Result<Option<String>, anyhow::Error> {
        // Look at the most recent segment first
        for segment in self.segments.iter().rev() {
            if let Some(k) = segment.get(key)? {
                return Ok(Some(k));
            }
        }

        Ok(None)
    }

    fn set(&mut self, key: &String, data: String) -> Result<(), anyhow::Error> {
        if let Some(segment) = self.segments.last_mut() {
            return segment.set(key, &LogEntry::Alive(data));
        }

        Err(anyhow!("No segment files"))
    }

    fn delete(&mut self, key: &String) -> Result<bool, anyhow::Error> {
        // Look at the most recent segment first
        for segment in self.segments.iter_mut().rev() {
            if segment.has_key(key) {
                return segment.delete(key);
            }
        }

        Ok(false)
    }
}

fn meme(hash_index: &HashIndex, key: &str) -> Result<(), anyhow::Error> {
    if let Some(val) = hash_index.get(&key.to_string())? {
        println!("The value for the key {} is: {}", key, val);
    } else {
        println!("No value exists for the key {}", key);
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let mut hash_index = HashIndex::new()?;
    hash_index.set(&"haha".to_string(), "HEHEHEHEHEHEHE".to_string())?;
    hash_index.set(
        &"hoho".to_string(),
        "abcd1e123laksdlasdaslkdjasldjasldajkld".to_string(),
    )?;

    meme(&hash_index, "haha")?;
    meme(&hash_index, "hoho")?;
    hash_index.delete(&"haha".to_string())?;
    meme(&hash_index, "haha")?;

    Ok(())
}
