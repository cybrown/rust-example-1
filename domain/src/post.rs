use crate::dependencies::DomainResult;
use crate::dependencies::PostDb;
use async_trait::async_trait;
use mockall::*;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[automock]
#[async_trait]
pub trait PostDomain {
    async fn get_post(&self, post_id: i32) -> DomainResult<Option<Post>>;
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>>;
    async fn create_post(&self, title: String, body: String) -> DomainResult<Post>;
    async fn publish_post(&self, post_id: i32) -> DomainResult<Option<Post>>;
    async fn unpublish_post(&self, post_id: i32) -> DomainResult<Option<Post>>;
}

pub fn new_post_domain(post_db: impl PostDb + Send + Sync) -> impl PostDomain {
    PostDomainImpl { post_db }
}

struct PostDomainImpl<P: PostDb> {
    post_db: P,
}

#[async_trait]
impl<P: PostDb + Send + Sync> PostDomain for PostDomainImpl<P> {
    async fn get_post(&self, post_id: i32) -> DomainResult<Option<Post>> {
        self.post_db.get_post_by_id(post_id).await
    }

    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<Post>> {
        self.post_db.get_posts(show_all).await
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<Post> {
        self.post_db.create_post(title, body).await
    }

    async fn publish_post(&self, post_id: i32) -> DomainResult<Option<Post>> {
        let post = self.post_db.get_post_by_id(post_id).await?;
        if let Some(post) = post.clone() {
            if !post.published {
                return self.post_db.post_set_published(post_id, true).await;
            }
        }
        Ok(post)
    }

    async fn unpublish_post(&self, post_id: i32) -> DomainResult<Option<Post>> {
        let post = self.post_db.get_post_by_id(post_id).await?;
        if let Some(post) = post.clone() {
            if post.published {
                return self.post_db.post_set_published(post_id, false).await;
            }
        }
        Ok(post)
    }
}
