//! A simple collection of ID related utilities.

use std::{
    env,
    time::{Duration, SystemTime},
};

lazy_static! {
    pub static ref ELUDRIS_EPOCH: SystemTime =
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_650_000_000);
}

/// An abstraction for generating spec-compliant IDs and handling incrementing them
///
/// ## Example
///
/// ```rust
/// use todel::ids::IdGenerator;
///
/// let mut generator = IdGenerator::new(); // Create a new ID generator.
///
/// generator.generate(); // Generate an ID which also increments the sequence.
/// ```
#[derive(Debug, Clone, Default)]
pub struct IdGenerator {
    sequence: u8,
    worker_id: u8,
}

impl IdGenerator {
    /// Create a new IdGenerator from an instance ID.
    pub fn new() -> Self {
        Self {
            sequence: 0,
            worker_id: env::var("ELUDRIS_WORKER_ID")
                .map(|v| {
                    v.parse()
                        .expect("Invalid ELUDRIS_WORKER_ID environment variable")
                })
                .unwrap_or(0),
        }
    }

    /// Generate a new ID and handle incrementing the sequence
    pub fn generate(&mut self) -> u64 {
        if self.sequence == u8::MAX {
            self.sequence = 0
        } else {
            self.sequence += 1;
        }
        SystemTime::now()
            .duration_since(*ELUDRIS_EPOCH)
            .expect("Couldn't get current timestamp")
            .as_secs()
            << 16
            | (self.worker_id as u64) << 8
            | self.sequence as u64
    }
}

#[cfg(test)]
mod tests {
    use super::IdGenerator;

    #[test]
    fn id_generator() {
        let mut generator = IdGenerator::new();

        let id = generator.generate();
        assert_eq!(id & 0xFF, 1);
        assert_eq!(id >> 8 & 0xFF, 0);

        let id = generator.generate();
        assert_eq!(id & 0xFF, 2);
        assert_eq!(id >> 8 & 0xFF, 0);
    }

    #[test]
    fn id_generator_overflow() {
        let mut generator = IdGenerator {
            sequence: u8::MAX - 1,
            worker_id: 3,
        };

        let id = generator.generate();
        assert_eq!(id & 0xFF, u8::MAX as u64);
        assert_eq!(id >> 8 & 0xFF, generator.worker_id as u64);

        let id = generator.generate();
        assert_eq!(id & 0xFF, 0);
        assert_eq!(id >> 8 & 0xFF, generator.worker_id as u64);

        let id = generator.generate();
        assert_eq!(id & 0xFF, 1);
        assert_eq!(id >> 8 & 0xFF, generator.worker_id as u64);
    }
}
