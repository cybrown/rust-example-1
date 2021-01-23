use mockall::predicate::*;
use mockall::*;

// Expected interface for a logger
#[automock]
pub trait Logger {
    fn log(&self, str: String);
}

// Expected interface for a dummy service to uppercase a string
#[automock]
pub trait Uppercaser {
    fn to_uppercase(&self, str: String) -> String;
}

// Expected interface for a counter
#[automock]
pub trait Counter {
    fn increment(&self);
    fn get_value(&self) -> i32;
}

// Main application
pub struct Application<U: Uppercaser, L: Logger, C: Counter> {
    uppercaser: U,
    logger: L,
    counter: C,
}

impl<U: Uppercaser, L: Logger, C: Counter> Application<U, L, C> {
    // A method that uses the dependencies
    pub fn run(&self) {
        self.logger.log("Start app !".to_owned());
        self.counter.increment();
        let k = "hello".to_owned();
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
    }

    // Injection through constructor
    pub fn new(uppercaser: U, logger: L, counter: C) -> Self {
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

    #[test]
    fn test_run() {
        let logger = {
            let mut logger = MockLogger::new();
            logger.expect_log().times(1).return_const(());
            logger
        };
        let uppercaser = {
            let mut mock = MockUppercaser::new();
            mock.expect_to_uppercase()
                .times(1)
                .return_const("A".to_owned());
            mock
        };
        let counter = {
            let mut mock = MockCounter::new();
            mock.expect_increment().times(1).return_const(());
            mock.expect_get_value().times(0);
            mock
        };
        let app = Application::new(uppercaser, logger, counter);

        app.run();
    }
}
