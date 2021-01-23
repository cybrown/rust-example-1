use std::rc::Rc;

// Expected interface for a logger
pub trait Logger {
    fn log(&self, str: String);
}

// Expected interface for a dummy service to uppercase a string
pub trait Uppercaser {
    fn to_uppercase(&self, str: String) -> String;
}

// Expected interface for a counter
pub trait Counter {
    fn increment(&self);
    fn get_value(&self) -> i32;
}

// Main application
pub struct Application {
    uppercaser: Rc<dyn Uppercaser>,
    logger: Rc<dyn Logger>,
    counter: Rc<dyn Counter>,
}

impl Application {
    // A method that uses the dependencies
    pub fn run(&self) {
        self.logger.log("Start app !".to_owned());
        self.counter.increment();
        let k = "hello".to_owned();
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
    }

    // Injection through constructor
    pub fn new(
        uppercaser: Rc<dyn Uppercaser>,
        logger: Rc<dyn Logger>,
        counter: Rc<dyn Counter>,
    ) -> Self {
        Self {
            uppercaser,
            logger,
            counter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct LoggerMock {
        pub called: RefCell<bool>,
    }

    impl LoggerMock {
        fn new() -> Self {
            Self {
                called: RefCell::new(false),
            }
        }
    }

    impl Logger for LoggerMock {
        fn log(&self, _: std::string::String) {
            *self.called.borrow_mut() = true;
        }
    }

    struct UppercaserMock {
        pub called: RefCell<bool>,
    }

    impl UppercaserMock {
        fn new() -> Self {
            Self {
                called: RefCell::new(false),
            }
        }
    }

    impl Uppercaser for UppercaserMock {
        fn to_uppercase(&self, _: std::string::String) -> std::string::String {
            *self.called.borrow_mut() = true;
            String::from("A")
        }
    }

    struct CounterMock {
        pub increment_called: RefCell<bool>,
        pub get_value_called: RefCell<bool>,
    }

    impl CounterMock {
        fn new() -> Self {
            Self {
                increment_called: RefCell::new(false),
                get_value_called: RefCell::new(false),
            }
        }
    }

    impl Counter for CounterMock {
        fn increment(&self) {
            *self.increment_called.borrow_mut() = true;
        }
        fn get_value(&self) -> i32 {
            *self.get_value_called.borrow_mut() = true;
            0
        }
    }

    #[test]
    fn test_run() {
        let logger = Rc::new(LoggerMock::new());
        let uppercaser = Rc::new(UppercaserMock::new());
        let counter = Rc::new(CounterMock::new());
        let app = Application::new(uppercaser.clone(), logger.clone(), counter.clone());

        app.run();

        assert_eq!(true, *logger.called.borrow());
        assert_eq!(true, *uppercaser.called.borrow());
        assert_eq!(true, *counter.increment_called.borrow());
        assert_eq!(false, *counter.get_value_called.borrow());
    }
}
