use crate::application::{Counter, Logger, Uppercaser as AppUppercaser};
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use std::sync::Mutex;

// Adapters to conform the external services to the expected interfaces by the application

impl Logger for PrintlnLogger {
    fn log(&self, line: std::string::String) {
        self.log(line)
    }
}

impl AppUppercaser for Uppercaser {
    fn to_uppercase(&self, str: String) -> String {
        return self.to_uppercase(str);
    }
}

pub struct CounterWrapper {
    counter: Mutex<SimpleCounter>,
}

impl CounterWrapper {
    pub fn new() -> CounterWrapper {
        CounterWrapper {
            counter: Mutex::new(SimpleCounter::new()),
        }
    }
}

impl Counter for CounterWrapper {
    fn increment(&self) {
        self.counter.lock().unwrap().increment()
    }

    fn get_value(&self) -> i32 {
        self.counter.lock().unwrap().get_value()
    }
}
