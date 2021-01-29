use crate::util::spawn_blocking;
use async_trait::async_trait;
use domain::AsyncPostDb;
use domain::DomainResult;
use domain::Post as DomainPost;
use domain::PostDb;
use domain::PostUpdates;

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
    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<DomainPost>> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.get_posts(show_all)).await
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<DomainPost> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.create_post(title, body)).await
    }

    async fn update_post(&self, post_id: i32, updates: PostUpdates) -> DomainResult<DomainPost> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || post_db.update_post(post_id, updates)).await
    }
}
