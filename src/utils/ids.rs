use std::sync::atomic::{AtomicU64, Ordering};

/// A sequential Id generator
pub struct IdGenerator {
    counter: AtomicU64,
}

impl IdGenerator {
    /// Create a new [`IdGenerator`]
    pub const fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    /// Generate next Id
    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
