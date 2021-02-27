use domain::{DomainResult, Post, PostDomain};

pub struct CreatePostCommand<D: PostDomain + Sync + Send> {
    post_domain: D,
}

impl<D: PostDomain + Sync + Send> CreatePostCommand<D> {
    pub fn new(post_domain: D) -> Self {
        Self { post_domain }
    }

    pub async fn run(&self) -> DomainResult<Post> {
        let res = self
            .post_domain
            .create_post(
                "from command".to_owned(),
                "body of post created from command".to_owned(),
            )
            .await;
        return res;
    }
}
