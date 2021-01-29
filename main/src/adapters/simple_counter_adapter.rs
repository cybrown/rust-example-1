use domain::Counter;
use simple_counter::SimpleCounter;
use std::sync::Mutex;

pub struct MutexCounterWrapper {
    simple_counter: Mutex<SimpleCounter>,
}

impl From<SimpleCounter> for MutexCounterWrapper {
    fn from(simple_counter: SimpleCounter) -> Self {
        Self {
            simple_counter: Mutex::new(simple_counter),
        }
    }
}

impl Counter for MutexCounterWrapper {
    fn increment(&self) {
        self.simple_counter.lock().unwrap().increment()
    }

    fn get_value(&self) -> i32 {
        self.simple_counter.lock().unwrap().get_value()
    }
}
