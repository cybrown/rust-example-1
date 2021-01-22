mod adapters;
mod application;
mod println_logger;
mod simple_counter;
mod uppercaser;

use adapters::{LoggerAdapter, MutexCounterWrapper, UppercaserAdapter};
use application::Application;
use println_logger::PrintlnLogger;
use simple_counter::SimpleCounter;
use std::rc::Rc;

fn main() {
    // Instantiate the shared dependencies
    let uppercaser: Rc<dyn application::Uppercaser> =
        Rc::new(UppercaserAdapter::from(uppercaser::Uppercaser {}));
    let logger: Rc<dyn application::Logger> =
        Rc::new(LoggerAdapter::from(PrintlnLogger::new(String::from("app"))));
    let counter: Rc<dyn application::Counter> =
        Rc::new(MutexCounterWrapper::from(SimpleCounter::new()));

    // Instantiate many applications who share the same dependencies
    let app1 = Application::new(uppercaser.clone(), logger.clone(), counter.clone());
    let app2 = Application::new(uppercaser.clone(), logger.clone(), counter.clone());

    // Run the applications with the same shared dependencies
    app1.run();
    app2.run();

    // Show how many time an app was run
    println!("Count: {}", counter.get_value());
}
