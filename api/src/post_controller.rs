use domain::DomainError;
use domain::PostDomain;
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

#[derive(Deserialize)]
pub struct WritePost {
    title: String,
    body: String,
}

impl PostController {
    pub fn new(post_db: Box<dyn PostDomain + Send + Sync>) -> Self {
        Self {
            post_db: Arc::from(post_db),
        }
    }

    pub async fn get_post(self, post_id: i32) -> Result<impl Reply, Rejection> {
        self.post_db
            .get_post(post_id)
            .await
            .map(|post| warp::reply::json(&post))
            .map_err(|err| warp::reject::custom(ApiError::from(err)))
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
            .map_err(|err| warp::reject::custom(ApiError::from(err)))
    }

    pub async fn create_post(self, post: WritePost) -> Result<impl Reply, Rejection> {
        self.post_db
            .create_post(post.title, post.body)
            .await
            .map(|p| warp::reply::json(&p))
            .map_err(|err| warp::reject::custom(ApiError::from(err)))
    }

    pub async fn publish_post(self, post_id: i32) -> Result<impl Reply, Rejection> {
        self.post_db
            .publish_post(post_id)
            .await
            .map(|p| warp::reply::json(&p))
            .map_err(|err| warp::reject::custom(ApiError::from(err)))
    }

    pub async fn unpublish_post(self, post_id: i32) -> Result<impl Reply, Rejection> {
        self.post_db
            .unpublish_post(post_id)
            .await
            .map(|p| warp::reply::json(&p))
            .map_err(|err| warp::reject::custom(ApiError::from(err)))
    }
}

#[derive(Debug)]
struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

impl warp::reject::Reject for ApiError {}
