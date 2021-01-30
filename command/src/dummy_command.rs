use domain::Counter;
use domain::Logger;
use domain::PostDomain;
use domain::Uppercaser;
use std::rc::Rc;

// Dummy command to run a process from the command line
pub struct DummyCommand<U: Uppercaser, L: Logger, P: PostDomain> {
    uppercaser: U,
    logger: L,
    counter: Rc<dyn Counter>,
    post_db: P,
}

impl<U: Uppercaser, L: Logger, P: PostDomain> DummyCommand<U, L, P> {
    // A method that uses the dependencies
    pub async fn run(&self) {
        self.logger.log("Start dummy command !".to_owned());
        self.counter.increment();
        let k = "hello".to_owned();
        let c = self.uppercaser.to_uppercase(k);
        println!("Hello: {}", c);
        self.post_db
            .get_posts(false)
            .await
            .map(|posts| {
                for post in posts {
                    println!("Post: {}", post.title);
                }
            })
            .unwrap();
        self.post_db
            .create_post("hello 2".to_owned(), "another body".to_owned())
            .await
            .unwrap();
    }

    // Injection through constructor
    pub fn new(uppercaser: U, logger: L, counter: Rc<dyn Counter>, post_db: P) -> Self {
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
    use domain::*;

    #[tokio::test]
    async fn test_run() {
        let dummy_command = DummyCommand::new(
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
                let mut mock = MockPostDomain::new();
                mock.expect_get_posts().returning(|_| Ok(vec![]));
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

        dummy_command.run().await;
    }
}
