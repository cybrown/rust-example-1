use crate::post::Post;
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;

#[automock]
#[async_trait]
pub trait PostDb {
    async fn get_post_by_id(&self, post_id: i32) -> DomainResult<Option<Post>>;
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>>;
    async fn create_post(&self, title: String, body: String) -> DomainResult<Post>;
    async fn post_set_published(&self, post_id: i32, published: bool)
        -> DomainResult<Option<Post>>;
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
