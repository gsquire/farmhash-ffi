use std::hash::Hasher;
use std::iter::Extend;

use libc::size_t;

extern "C" {
    fn Hash64(s: *const u8, len: size_t) -> u64;
    fn Hash64WithSeed(s: *const u8, len: size_t, seed: u64) -> u64;
}

// Given a slice of bytes, return its 64-bit hash.
fn hash_64(input: &[u8]) -> u64 {
    unsafe { Hash64(input.as_ptr(), input.len() as size_t) }
}

// Given a slice of bytes and a seed, return its 64-bit hash.
fn hash_64_with_seed(input: &[u8], seed: u64) -> u64 {
    unsafe { Hash64WithSeed(input.as_ptr(), input.len() as size_t, seed) }
}

/// `Hash64` implements the 64-bit version of FarmHash.
#[derive(Debug, Default)]
pub struct Hash64 {
    buf: Vec<u8>,
    seed: Option<u64>,
}

impl Hash64 {
    /// Use a `Hash64` with a pre-defined seed.
    pub fn with_seed(seed: u64) -> Hash64 {
        Hash64 {
            buf: Default::default(),
            seed: Some(seed),
        }
    }
}

impl Hasher for Hash64 {
    fn write(&mut self, bytes: &[u8]) {
        self.buf.extend(bytes);
    }

    fn finish(&self) -> u64 {
        if let Some(s) = self.seed {
            hash_64_with_seed(&self.buf, s)
        } else {
            hash_64(&self.buf)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use super::Hash64;

    #[test]
    fn no_seed_64() {
        const EXPECTED: u64 = 18084869015357591102;

        let mut h = Hash64::default();
        h.write(b"farmhash");

        assert_eq!(EXPECTED, h.finish());
    }

    #[test]
    fn seed_64() {
        const EXPECTED: u64 = 15795874029479571151;

        let mut h = Hash64::with_seed(7);
        h.write(b"farmhash");

        assert_eq!(EXPECTED, h.finish());
    }
}
