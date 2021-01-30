use crate::util::spawn_blocking;
use async_trait::async_trait;
use db_diesel::DieselPostDb;
use db_diesel::GetPostsCriteria;
use db_diesel::Post;
use db_diesel::UpdatePost;
use domain::DomainResult;
use domain::Post as DomainPost;
use domain::{DomainError, PostDb};

#[derive(Clone)]
pub struct PostDbWrapper {
    post_db: DieselPostDb,
}

impl From<DieselPostDb> for PostDbWrapper {
    fn from(post_db: DieselPostDb) -> Self {
        PostDbWrapper { post_db }
    }
}

#[async_trait]
impl PostDb for PostDbWrapper {
    async fn get_post_by_id(&self, post_id: i32) -> DomainResult<Option<DomainPost>> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || {
            post_db
                .get_post_by_id(post_id)
                .map(|post| post.map(|post| db_post_to_app_post(&post)))
                .map_err(|_| DomainError::new("failed to get post".to_owned()))
        })
        .await
    }

    async fn get_posts(&self, show_all: bool) -> DomainResult<Vec<DomainPost>> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || {
            post_db
                .get_posts(GetPostsCriteria {
                    published: if show_all { None } else { Some(true) },
                })
                .map(|posts| {
                    posts
                        .iter()
                        .map(|post| db_post_to_app_post(post))
                        .collect::<Vec<DomainPost>>()
                })
                .map_err(|_| DomainError::new("failed to get posts".to_owned()))
        })
        .await
    }

    async fn create_post(&self, title: String, body: String) -> DomainResult<DomainPost> {
        let post_db = self.post_db.clone();
        spawn_blocking(move || {
            post_db
                .insert_post(title, body)
                .map(|post| db_post_to_app_post(&post))
                .map_err(|_| DomainError::new("failed to create post".to_owned()))
        })
        .await
    }

    async fn post_set_published(
        &self,
        post_id: i32,
        published: bool,
    ) -> DomainResult<Option<DomainPost>> {
        self.post_db
            .update_post(
                post_id,
                UpdatePost {
                    body: None,
                    title: None,
                    published: Some(published),
                },
            )
            .map(|post| post.map(|post| db_post_to_app_post(&post)))
            .map_err(|_| DomainError::new("failed to get posts".to_owned()))
    }
}

fn db_post_to_app_post(db_post: &Post) -> DomainPost {
    DomainPost {
        id: db_post.id,
        title: db_post.title.to_owned(),
        body: db_post.body.to_owned(),
        published: db_post.published,
    }
}
