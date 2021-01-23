use crate::db::Post;
use mockall::predicate::*;
use mockall::*;
use std::rc::Rc;

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

#[automock]
pub trait PostDao {
    fn get_posts(&self) -> Result<Vec<Post>, AppError>;
}

#[derive(Debug)]
pub struct AppError;

impl std::convert::From<diesel::result::Error> for AppError {
    fn from(_: diesel::result::Error) -> Self {
        Self {}
    }
}

// Main application
pub struct Application<U: Uppercaser, L: Logger, P: PostDao> {
    uppercaser: U,
    logger: L,
    counter: Rc<dyn Counter>,
    post_dao: P,
}

impl<U: Uppercaser, L: Logger, P: PostDao> Application<U, L, P> {
    // A method that uses the dependencies
    pub fn run(&self) {
        self.logger.log("Start app !".to_owned());
        self.counter.increment();
        let k = "hello".to_owned();
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
        self.post_dao
            .get_posts()
            .map(|posts| {
                for post in posts {
                    println!("Post: {}", post.title);
                }
            })
            .unwrap();
    }

    // Injection through constructor
    pub fn new(uppercaser: U, logger: L, counter: Rc<dyn Counter>, post_dao: P) -> Self {
        Self {
            uppercaser,
            logger,
            counter,
            post_dao,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let app = Application::new(
            {
                let mut mock = MockUppercaser::new();
                mock.expect_to_uppercase()
                    .times(1)
                    .return_const("A".to_owned());
                mock
            },
            {
                let mut logger = MockLogger::new();
                logger.expect_log().times(1).return_const(());
                logger
            },
            {
                let mut mock = MockCounter::new();
                mock.expect_increment().times(1).return_const(());
                mock.expect_get_value().times(0);
                Rc::new(mock)
            },
            {
                let mut mock = MockPostDao::new();
                mock.expect_get_posts().returning(|| Ok(vec![]));
                mock
            },
        );

        app.run();
    }
}
