use crate::domain::AsyncPostDb;
use crate::domain::DomainResult;
use crate::domain::PostUpdates;
use async_trait::async_trait;
use mockall::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[automock]
#[async_trait]
pub trait PostDomain {
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>>;
    async fn create_post(&self, title: String, body: String) -> DomainResult<Post>;
    async fn publish_post(&self, post_id: i32) -> DomainResult<Post>;
}

pub fn new_post_domain(post_db: Box<dyn AsyncPostDb + Send + Sync>) -> impl PostDomain {
    PostDomainImpl { post_db }
}

struct PostDomainImpl {
    post_db: Box<dyn AsyncPostDb + Send + Sync>,
}

#[async_trait]
impl PostDomain for PostDomainImpl {
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>> {
        self.post_db.get_posts(show_all).await
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<Post> {
        self.post_db.create_post(title, body).await
    }

    async fn publish_post(&self, post_id: i32) -> DomainResult<Post> {
        self.post_db
            .update_post(
                post_id,
                PostUpdates {
                    published: Some(true),
                },
            )
            .await
    }
}
