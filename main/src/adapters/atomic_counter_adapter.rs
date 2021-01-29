use atomic_counter::AtomicCounter;
use domain::Counter;

pub struct AtomicCounterAdapter {
    atomic_counter: AtomicCounter,
}

impl From<AtomicCounter> for AtomicCounterAdapter {
    fn from(atomic_counter: AtomicCounter) -> Self {
        Self {
            atomic_counter: atomic_counter,
        }
    }
}

impl Counter for AtomicCounterAdapter {
    fn increment(&self) {
        self.atomic_counter.increment()
    }

    fn get_value(&self) -> i32 {
        self.atomic_counter.get_value()
    }
}
