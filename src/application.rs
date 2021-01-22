use std::rc::Rc;

// Expected interface for a logger
pub trait Logger {
    fn log(&self, str: String);
}

// Expected interface for a dummy service to uppercase a string
pub trait Uppercaser {
    fn to_uppercase(&self, str: String) -> String;
}

// Main application
pub struct Application {
    uppercaser: Rc<dyn Uppercaser>,
    logger: Rc<dyn Logger>,
}

impl Application {
    // A method that uses the dependencies
    pub fn run(&self) {
        self.logger.log(String::from("Start app !"));
        let k: String = String::from("hello");
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
    }

    // Injection through constructor
    pub fn new(uppercaser: Rc<dyn Uppercaser>, logger: Rc<dyn Logger>) -> Application {
        Application { uppercaser, logger }
    }
}
