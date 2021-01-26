use crate::application::AppError;
use crate::application::PostDb;
use crate::application::{Counter, Logger, Post as AppPost, Uppercaser as AppUppercaser};
use crate::atomic_counter::AtomicCounter;
use crate::diesel_post_db::DieselPostDb;
use crate::diesel_post_db::Post;
use crate::post_controller::AsyncPostDb;
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use async_trait::async_trait;
use std::sync::Mutex;
use tokio::task;

// Adapters to conform the external services to the expected interfaces by the application

// Logger

pub struct LoggerAdapter {
    println_logger: PrintlnLogger,
}

impl From<PrintlnLogger> for LoggerAdapter {
    fn from(println_logger: PrintlnLogger) -> Self {
        Self {
            println_logger: println_logger,
        }
    }
}

impl Logger for LoggerAdapter {
    fn log(&self, line: String) {
        self.println_logger.log(line)
    }
}

// Uppercaser

pub struct UppercaserAdapter {
    uppercaser: Uppercaser,
}

impl From<Uppercaser> for UppercaserAdapter {
    fn from(uppercaser: Uppercaser) -> Self {
        Self { uppercaser }
    }
}

impl AppUppercaser for UppercaserAdapter {
    fn to_uppercase(&self, str: String) -> String {
        self.uppercaser.to_uppercase(str)
    }
}

// Counter with Mutex

pub struct MutexCounterWrapper {
    simple_counter: Mutex<SimpleCounter>,
}

impl From<SimpleCounter> for MutexCounterWrapper {
    fn from(simple_counter: SimpleCounter) -> Self {
        Self {
            simple_counter: Mutex::new(simple_counter),
        }
    }
}

impl Counter for MutexCounterWrapper {
    fn increment(&self) {
        self.simple_counter.lock().unwrap().increment()
    }

    fn get_value(&self) -> i32 {
        self.simple_counter.lock().unwrap().get_value()
    }
}

pub struct AsyncPostDbWrapper {
    post_db: DieselPostDb,
}

impl From<DieselPostDb> for AsyncPostDbWrapper {
    fn from(post_db: DieselPostDb) -> Self {
        AsyncPostDbWrapper { post_db }
    }
}

#[async_trait]
impl AsyncPostDb for AsyncPostDbWrapper {
    async fn get_posts(&self) -> Result<Vec<AppPost>, AppError> {
        let post_db = self.post_db.clone();
        task::spawn_blocking(move || {
            post_db
                .get_posts()
                .map(|posts| {
                    posts
                        .iter()
                        .map(|post| db_post_to_app_post(post))
                        .collect::<Vec<AppPost>>()
                })
                .map_err(|_| AppError {})
        })
        .await
        .map_err(|_| AppError {})?
    }

    async fn create_post(
        &self,
        title: String,
        body: String,
    ) -> std::result::Result<AppPost, AppError> {
        let post_db = self.post_db.clone();
        task::spawn_blocking(move || {
            post_db
                .insert_post(title, body)
                .map(|post| db_post_to_app_post(&post))
                .map_err(|_| AppError {})
        })
        .await
        .map_err(|_| AppError {})?
    }
}

pub struct PostDbWrapper {
    post_db: DieselPostDb,
}

impl From<DieselPostDb> for PostDbWrapper {
    fn from(post_db: DieselPostDb) -> Self {
        PostDbWrapper { post_db }
    }
}

impl PostDb for PostDbWrapper {
    fn get_posts(&self) -> Result<Vec<AppPost>, AppError> {
        self.post_db
            .get_posts()
            .map(|posts| {
                posts
                    .iter()
                    .map(|post| db_post_to_app_post(post))
                    .collect::<Vec<AppPost>>()
            })
            .map_err(|_| AppError {})
    }

    fn create_post(&self, title: String, body: String) -> std::result::Result<AppPost, AppError> {
        self.post_db
            .insert_post(title, body)
            .map(|post| db_post_to_app_post(&post))
            .map_err(|_| AppError {})
    }
}

fn db_post_to_app_post(db_post: &Post) -> AppPost {
    AppPost {
        id: db_post.id,
        title: db_post.title.to_owned(),
        body: db_post.body.to_owned(),
        published: db_post.published,
    }
}

// Counter with Atomic

pub struct AtomicCounterAdapter {
    atomic_counter: AtomicCounter,
}

impl From<AtomicCounter> for AtomicCounterAdapter {
    fn from(atomic_counter: AtomicCounter) -> Self {
        Self {
            atomic_counter: atomic_counter,
        }
    }
}

impl Counter for AtomicCounterAdapter {
    fn increment(&self) {
        self.atomic_counter.increment()
    }

    fn get_value(&self) -> i32 {
        self.atomic_counter.get_value()
    }
}
