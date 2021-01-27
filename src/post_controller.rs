use crate::application::AppError;
use crate::application::Post;
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use serde::Deserialize;
use std::sync::Arc;
use warp::Rejection;
use warp::Reply;

#[automock]
#[async_trait]
pub trait AsyncPostDb {
    async fn get_posts(&self, show_all: bool) -> Result<Vec<Post>, AppError>;
    async fn create_post(&self, title: String, body: String) -> Result<Post, AppError>;
}

#[derive(Clone)]
pub struct PostController {
    post_db: Arc<dyn AsyncPostDb + Send + Sync>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct QueryParameters {
    showAll: Option<bool>,
}

impl PostController {
    pub fn new(post_db: Box<dyn AsyncPostDb + Send + Sync>) -> Self {
        Self {
            post_db: Arc::from(post_db),
        }
    }

    pub async fn get_posts(self, query: QueryParameters) -> Result<impl Reply, Rejection> {
        let show_all = match query.showAll {
            Some(a) => a,
            _ => false,
        };
        self.post_db
            .get_posts(show_all)
            .await
            .map(|posts| warp::reply::json(&posts))
            .map_err(|err| warp::reject::custom(err))
    }

    pub async fn create_post(self) -> Result<impl Reply, Rejection> {
        self.post_db
            .create_post("title".to_owned(), "body".to_owned())
            .await
            .map(|p| warp::reply::json(&p))
            .map_err(|err| warp::reject::custom(err))
    }
}

impl warp::reject::Reject for AppError {}
