use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRTSTUVWXYZabcdefghijklmnopqrtstuvwxyz0123456789-";

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    let mut rng = Xorshift32::seed_from_u64(seed);

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len() as u32);
            CHARSET[idx as usize] as char
        })
        .collect()
}

struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    fn seed_from_u64(seed: u64) -> Xorshift32 {
        Xorshift32 { state: seed as u32 }
    }

    fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;

        self.state.wrapping_rem(range.end - range.start) + range.start
    }
}
