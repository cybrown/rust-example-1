use crate::db_diesel::DieselPostDb;
use crate::db_diesel::GetPostsCriteria;
use crate::db_diesel::Post;
use crate::db_diesel::UpdatePost;
use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::Post as DomainPost;
use crate::domain::PostDb;
use crate::domain::PostUpdates;

#[derive(Clone)]
pub struct PostDbWrapper {
    post_db: DieselPostDb,
}

impl From<DieselPostDb> for PostDbWrapper {
    fn from(post_db: DieselPostDb) -> Self {
        PostDbWrapper { post_db }
    }
}

impl PostDb for PostDbWrapper {
    fn get_posts(&self, show_all: bool) -> DomainResult<Vec<DomainPost>> {
        self.post_db
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
    }

    fn create_post(&self, title: String, body: String) -> DomainResult<DomainPost> {
        self.post_db
            .insert_post(title, body)
            .map(|post| db_post_to_app_post(&post))
            .map_err(|_| DomainError::new("failed to create post".to_owned()))
    }

    fn update_post(&self, post_id: i32, updates: PostUpdates) -> DomainResult<DomainPost> {
        self.post_db
            .update_post(
                post_id,
                UpdatePost {
                    body: None,
                    title: None,
                    published: updates.published,
                },
            )
            .map(|post| db_post_to_app_post(&post))
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
