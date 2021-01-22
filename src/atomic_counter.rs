// An alternative implementation of a Counter service

use std::sync::atomic::{AtomicI32, Ordering};

pub struct AtomicCounter {
    value: AtomicI32,
}

impl AtomicCounter {
    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    pub fn new() -> AtomicCounter {
        AtomicCounter {
            value: AtomicI32::new(0),
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let counter = AtomicCounter {
            value: AtomicI32::new(0),
        };

        counter.increment();

        assert_eq!(counter.get_value(), 1);
    }

    #[test]
    fn test_get_value() {
        let counter = AtomicCounter {
            value: AtomicI32::new(42),
        };

        assert_eq!(counter.get_value(), 42);
    }
}
