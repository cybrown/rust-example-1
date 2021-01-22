use crate::application::{Counter, Logger, Uppercaser as AppUppercaser};
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use crate::AtomicCounter;
use std::sync::Mutex;

// Adapters to conform the external services to the expected interfaces by the application

pub struct LoggerAdapter(PrintlnLogger);

impl LoggerAdapter {
    pub fn from(println_logger: PrintlnLogger) -> LoggerAdapter {
        LoggerAdapter(println_logger)
    }
}

impl Logger for LoggerAdapter {
    fn log(&self, line: std::string::String) {
        self.0.log(line)
    }
}

pub struct UppercaserAdapter(Uppercaser);

impl UppercaserAdapter {
    pub fn from(uppercaser: Uppercaser) -> UppercaserAdapter {
        UppercaserAdapter(uppercaser)
    }
}

impl AppUppercaser for UppercaserAdapter {
    fn to_uppercase(&self, str: String) -> String {
        return self.0.to_uppercase(str);
    }
}

pub struct MutexCounterWrapper(Mutex<SimpleCounter>);

impl MutexCounterWrapper {
    pub fn from(simple_counter: SimpleCounter) -> MutexCounterWrapper {
        MutexCounterWrapper(Mutex::new(simple_counter))
    }
}

impl Counter for MutexCounterWrapper {
    fn increment(&self) {
        self.0.lock().unwrap().increment()
    }

    fn get_value(&self) -> i32 {
        self.0.lock().unwrap().get_value()
    }
}

pub struct AtomicCounterAdapter(AtomicCounter);

impl AtomicCounterAdapter {
    pub fn from(atomic_counter: AtomicCounter) -> AtomicCounterAdapter {
        AtomicCounterAdapter(atomic_counter)
    }
}

impl Counter for AtomicCounterAdapter {
    fn increment(&self) {
        self.0.increment()
    }

    fn get_value(&self) -> i32 {
        self.0.get_value()
    }
}
