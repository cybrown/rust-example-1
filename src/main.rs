mod adapters;
mod application;
mod println_logger;
mod simple_counter;
mod uppercaser;

use adapters::{LoggerAdapter, MutexCounterWrapper, UppercaserAdapter};
use application::{Application, Counter};
use println_logger::PrintlnLogger;
use simple_counter::SimpleCounter;

fn main() {
    // Instantiate the shared dependencies
    let uppercaser = UppercaserAdapter::from(uppercaser::Uppercaser {});
    let logger = LoggerAdapter::from(PrintlnLogger::new("app".to_owned()));
    let counter = MutexCounterWrapper::from(SimpleCounter::new());

    // Instantiate many applications who share the same dependencies
    let app1 = Application::new(uppercaser, logger.clone(), counter.clone());
    let app2 = Application::new(uppercaser, logger.clone(), counter.clone());

    // Run the applications with the same shared dependencies
    app1.run();
    app2.run();

    // Show how many time an app was run
    println!("Count: {}", counter.get_value());
}
