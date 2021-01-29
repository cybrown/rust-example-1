use crate::application::AppError;
use crate::application::AppResult;
use crate::application::PostDb;
use crate::application::{Counter, Logger, Post as AppPost, Uppercaser as AppUppercaser};
use crate::atomic_counter::AtomicCounter;
use crate::diesel_post_db::DieselPostDb;
use crate::diesel_post_db::GetPostsCriteria;
use crate::diesel_post_db::Post;
use crate::diesel_post_db::UpdatePost;
use crate::post_controller::AsyncPostDb;
use crate::post_controller::PostUpdates;
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use crate::util::spawn_blocking;
use async_trait::async_trait;
use std::sync::Mutex;

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

pub struct AsyncPostDbWrapper<P: PostDb + Clone + Send + Sync + 'static> {
    post_db: P,
}

impl<P: PostDb + Clone + Send + Sync + 'static> AsyncPostDbWrapper<P> {
    pub fn new(post_db: P) -> Self {
        AsyncPostDbWrapper { post_db }
    }
}

#[async_trait]
impl<P: PostDb + Clone + Send + Sync + 'static> AsyncPostDb for AsyncPostDbWrapper<P> {
    async fn get_posts(&self, show_all: bool) -> AppResult<Vec<AppPost>> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.get_posts(show_all)).await
    }

    async fn create_post(&self, title: String, body: String) -> AppResult<AppPost> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.create_post(title, body)).await
    }

    async fn update_post(&self, post_id: i32, updates: PostUpdates) -> AppResult<AppPost> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.update_post(post_id, updates)).await
    }
}

#[derive(Clone)]
pub struct PostDbWrapper {
    post_db: DieselPostDb,
}

impl From<DieselPostDb> for PostDbWrapper {
    fn from(post_db: DieselPostDb) -> Self {
        PostDbWrapper { post_db }
    }
}

impl PostDb for PostDbWrapper {
    fn get_posts(&self, show_all: bool) -> AppResult<Vec<AppPost>> {
        self.post_db
            .get_posts(GetPostsCriteria {
                published: if show_all { None } else { Some(true) },
            })
            .map(|posts| {
                posts
                    .iter()
                    .map(|post| db_post_to_app_post(post))
                    .collect::<Vec<AppPost>>()
            })
            .map_err(|_| AppError::new("failed to get posts".to_owned()))
    }

    fn create_post(&self, title: String, body: String) -> AppResult<AppPost> {
        self.post_db
            .insert_post(title, body)
            .map(|post| db_post_to_app_post(&post))
            .map_err(|_| AppError::new("failed to create post".to_owned()))
    }

    fn update_post(&self, post_id: i32, updates: PostUpdates) -> AppResult<AppPost> {
        self.post_db
            .update_post(
                post_id,
                UpdatePost {
                    body: None,
                    title: None,
                    published: updates.published,
                },
            )
            .map(|post| db_post_to_app_post(&post))
            .map_err(|_| AppError::new("failed to get posts".to_owned()))
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
