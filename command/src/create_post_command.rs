use domain::{DomainResult, Post, PostDomain};

pub struct CreatePostCommand<D>
where
    D: PostDomain + Sync + Send,
{
    post_domain: D,
}

impl<D> CreatePostCommand<D>
where
    D: PostDomain + Sync + Send,
{
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
