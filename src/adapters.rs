use crate::application::{Counter, Logger, Uppercaser as AppUppercaser};
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use std::rc::Rc;
use std::sync::Mutex;

// Adapters to conform the external services to the expected interfaces by the application

// Logger

#[derive(Clone)]
pub struct LoggerAdapter(PrintlnLogger);

impl From<PrintlnLogger> for LoggerAdapter {
    fn from(println_logger: PrintlnLogger) -> Self {
        Self(println_logger)
    }
}

impl Logger for LoggerAdapter {
    fn log(&self, line: String) {
        self.0.log(line)
    }
}

// Uppercaser

#[derive(Copy, Clone)]
pub struct UppercaserAdapter(Uppercaser);

impl From<Uppercaser> for UppercaserAdapter {
    fn from(uppercaser: Uppercaser) -> Self {
        Self(uppercaser)
    }
}

impl AppUppercaser for UppercaserAdapter {
    fn to_uppercase(&self, str: String) -> String {
        self.0.to_uppercase(str)
    }
}

// Counter with Mutex

#[derive(Clone)]
pub struct MutexCounterWrapper(Rc<Mutex<SimpleCounter>>);

impl From<SimpleCounter> for MutexCounterWrapper {
    fn from(simple_counter: SimpleCounter) -> Self {
        Self(Rc::new(Mutex::new(simple_counter)))
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

/*
// Counter with Atomic

pub struct AtomicCounterAdapter(AtomicCounter);

impl From<AtomicCounter> for AtomicCounterAdapter {
    fn from(atomic_counter: AtomicCounter) -> Self {
        Self(atomic_counter)
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
*/
