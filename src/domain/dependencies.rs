use crate::domain::Post;
use async_trait::async_trait;
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

pub struct PostUpdates {
    pub published: Option<bool>,
}

#[automock]
pub trait PostDb {
    fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>>;
    fn create_post(&self, title: String, body: String) -> DomainResult<Post>;
    fn update_post(&self, post_id: i32, updates: PostUpdates) -> DomainResult<Post>;
}

#[automock]
#[async_trait]
pub trait AsyncPostDb {
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>>;
    async fn create_post(&self, title: String, body: String) -> DomainResult<Post>;
    async fn update_post(&self, post_id: i32, updates: PostUpdates) -> DomainResult<Post>;
}

#[derive(Debug)]
pub struct DomainError {
    message: String,
}

pub type DomainResult<T> = Result<T, DomainError>;

impl DomainError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}