use super::segment::{LogEntry, Segment};
use anyhow::anyhow;

pub struct HashIndex {
    segments: Vec<Segment>,
}

impl HashIndex {
    pub fn new() -> Result<Self, anyhow::Error> {
        let segment = Segment::new()?;
        let hash_index = HashIndex {
            segments: vec![segment],
        };

        Ok(hash_index)
    }

    pub fn get(&self, key: &String) -> Result<Option<String>, anyhow::Error> {
        // Look at the most recent segment first
        for segment in self.segments.iter().rev() {
            if let Some(k) = segment.get(key)? {
                return Ok(Some(k));
            }
        }

        Ok(None)
    }

    pub fn set(&mut self, key: &String, data: String) -> Result<(), anyhow::Error> {
        if let Some(segment) = self.segments.last_mut() {
            return segment.set(key, &LogEntry::Alive(data));
        }

        Err(anyhow!("No segment files"))
    }

    pub fn delete(&mut self, key: &String) -> Result<bool, anyhow::Error> {
        // Look at the most recent segment first
        for segment in self.segments.iter_mut().rev() {
            if segment.has_key(key) {
                return segment.delete(key);
            }
        }

        Ok(false)
    }
}
