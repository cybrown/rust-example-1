mod adapters;
mod application;
mod println_logger;
mod service_registry;
mod simple_counter;
mod uppercaser;

use crate::service_registry::ServiceRegistry;
use adapters::{LoggerAdapter, MutexCounterWrapper, UppercaserAdapter};
use application::{Application, Counter};
use println_logger::PrintlnLogger;
use simple_counter::SimpleCounter;

fn main() {
    let sr = ServiceRegistry::new();

    // Instantiate many applications who share the same dependencies
    let app1 = sr.get_application();
    let app2 = sr.get_application();

    // Run the applications with the same shared dependencies
    app1.run();
    app2.run();

    // Show how many time an app was run
    println!("Count: {}", sr.get_counter().get_value());
}
