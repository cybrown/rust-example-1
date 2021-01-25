use crate::application::AppError;
use crate::application::Post;
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use std::sync::Arc;
use warp::Rejection;
use warp::Reply;

#[automock]
#[async_trait]
pub trait AsyncPostDb {
    async fn get_posts(&self) -> Result<Vec<Post>, AppError>;
    fn create_post(&self, title: String, body: String) -> Result<Post, AppError>;
}

#[derive(Clone)]
pub struct PostController {
    post_db: Arc<dyn AsyncPostDb + Send + Sync>,
}

impl PostController {
    pub fn new(post_db: Arc<dyn AsyncPostDb + Send + Sync>) -> Self {
        Self { post_db }
    }

    pub async fn get_posts(self) -> Result<impl Reply, Rejection> {
        self.post_db
            .get_posts()
            .await
            .map(|posts| warp::reply::json(&posts))
            .map_err(|_| warp::reject::not_found())
    }
}
