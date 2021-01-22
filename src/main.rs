mod adapters;
mod application;
mod println_logger;
mod uppercaser;

use application::Application;
use println_logger::PrintlnLogger;
use std::rc::Rc;

fn main() {
    // Instantiate the shared dependencies
    let uppercaser: Rc<dyn application::Uppercaser> = Rc::new(uppercaser::Uppercaser {});
    let logger: Rc<dyn application::Logger> = Rc::new(PrintlnLogger::new(String::from("app")));

    // Instantiate many applications who share the same dependencies
    let app1 = Application::new(&uppercaser, &logger);
    let app2 = Application::new(&uppercaser, &logger);

    // Run the applications with the same shared dependencies
    app1.run();
    app2.run();
}
