use mockall::predicate::*;
use mockall::*;
use serde::Serialize;

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

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[automock]
pub trait PostDb {
    fn get_posts(&self) -> Result<Vec<Post>, AppError>;
    fn create_post(&self, title: String, body: String) -> Result<Post, AppError>;
}

#[derive(Debug)]
pub struct AppError;

impl From<diesel::result::Error> for AppError {
    fn from(_: diesel::result::Error) -> Self {
        Self {}
    }
}

// Main application
pub struct Application<U: Uppercaser, L: Logger, P: PostDb> {
    uppercaser: U,
    logger: L,
    counter: Box<dyn Counter>,
    post_db: P,
}

impl<U: Uppercaser, L: Logger, P: PostDb> Application<U, L, P> {
    // A method that uses the dependencies
    pub fn run(&self) {
        self.logger.log("Start app !".to_owned());
        self.counter.increment();
        let k = "hello".to_owned();
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
        self.post_db
            .get_posts()
            .map(|posts| {
                for post in posts {
                    println!("Post: {}", post.title);
                }
            })
            .unwrap();
        self.post_db
            .create_post("hello 2".to_owned(), "another body".to_owned())
            .unwrap();
    }

    // Injection through constructor
    pub fn new(uppercaser: U, logger: L, counter: Box<dyn Counter>, post_db: P) -> Self {
        Self {
            uppercaser,
            logger,
            counter,
            post_db,
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
                let mut mock = Box::new(MockCounter::new());
                mock.expect_increment().times(1).return_const(());
                mock.expect_get_value().times(0);
                mock
            },
            {
                let mut mock = MockPostDb::new();
                mock.expect_get_posts().returning(|| Ok(vec![]));
                mock.expect_create_post().returning(|_, _| {
                    Ok(Post {
                        id: 0,
                        title: "".to_owned(),
                        body: "".to_owned(),
                        published: false,
                    })
                });
                mock
            },
        );

        app.run();
    }
}
