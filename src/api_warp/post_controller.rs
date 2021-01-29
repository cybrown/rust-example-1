use crate::domain::DomainError;
use crate::domain::PostDomain;
use serde::Deserialize;
use std::sync::Arc;
use warp::Rejection;
use warp::Reply;

#[derive(Clone)]
pub struct PostController {
    post_db: Arc<dyn PostDomain + Send + Sync>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct QueryParameters {
    show_all: Option<bool>,
}

impl PostController {
    pub fn new(post_db: Box<dyn PostDomain + Send + Sync>) -> Self {
        Self {
            post_db: Arc::from(post_db),
        }
    }

    pub async fn get_posts(self, query: QueryParameters) -> Result<impl Reply, Rejection> {
        let show_all = match query.show_all {
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

    pub async fn publish_post(self, post_id: i32) -> Result<impl Reply, Rejection> {
        self.post_db
            .publish_post(post_id)
            .await
            .map(|p| warp::reply::json(&p))
            .map_err(|err| warp::reject::custom(err))
    }
}

impl warp::reject::Reject for DomainError {}
