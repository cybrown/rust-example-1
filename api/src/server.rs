use crate::ApiError;
use domain::PostDomain;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{
    any, body::json, delete, get, hyper::StatusCode, path, path::end, post, put, query, reject,
    reply, serve, Filter, Rejection, Reply,
};

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    dbg!(err);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn run_server(post_domain: Box<dyn PostDomain + Send + Sync>) {
    let posts_api = {
        let arc_post_domain: Arc<dyn PostDomain + Send + Sync> = Arc::from(post_domain);
        let with_post_domain = any().map(move || arc_post_domain.clone());

        async fn get_post_handler(
            post_domain: Arc<dyn PostDomain + Send + Sync>,
            post_id: i32,
        ) -> Result<impl Reply, Rejection> {
            post_domain
                .get_post(post_id)
                .await
                .map(|post| reply::json(&post))
                .map_err(|err| reject::custom(ApiError::from(err)))
        }

        let get_post = get()
            .and(with_post_domain.clone())
            .and(path!(i32))
            .and_then(get_post_handler);

        let get_posts = get()
            .and(with_post_domain.clone())
            .and(end())
            .and(query())
            .and_then(get_posts_handler);

        #[derive(Deserialize, Debug, Copy, Clone)]
        pub struct QueryParameters {
            show_all: Option<bool>,
        }

        async fn get_posts_handler(
            post_domain: Arc<dyn PostDomain + Send + Sync>,
            query: QueryParameters,
        ) -> Result<impl Reply, Rejection> {
            post_domain
                .get_posts(query.show_all.unwrap_or(false))
                .await
                .map(|posts| reply::json(&posts))
                .map_err(|err| reject::custom(ApiError::from(err)))
        }

        #[derive(Deserialize)]
        pub struct WritePost {
            title: String,
            body: String,
        }

        async fn create_post_handler(
            post_domain: Arc<dyn PostDomain + Send + Sync>,
            post: WritePost,
        ) -> Result<impl Reply, Rejection> {
            post_domain
                .create_post(post.title, post.body)
                .await
                .map(|p| reply::with_status(reply::json(&p), StatusCode::CREATED))
                .map_err(|err| reject::custom(ApiError::from(err)))
        }

        let create_post = post()
            .and(with_post_domain.clone())
            .and(json())
            .and(end())
            .and_then(create_post_handler);

        async fn publish_post_handler(
            post_domain: Arc<dyn PostDomain + Send + Sync>,
            post_id: i32,
        ) -> Result<impl Reply, Rejection> {
            post_domain
                .publish_post(post_id)
                .await
                .map(|p| reply::json(&p))
                .map_err(|err| reject::custom(ApiError::from(err)))
        }

        let publish_post = put()
            .and(with_post_domain.clone())
            .and(path!(i32 / "published"))
            .and_then(publish_post_handler);

        async fn unpublish_post_handler(
            post_domain: Arc<dyn PostDomain + Send + Sync>,
            post_id: i32,
        ) -> Result<impl Reply, Rejection> {
            post_domain
                .unpublish_post(post_id)
                .await
                .map(|p| reply::json(&p))
                .map_err(|err| reject::custom(ApiError::from(err)))
        }

        let unpublish_post = delete()
            .and(with_post_domain.clone())
            .and(path!(i32 / "published"))
            .and_then(unpublish_post_handler);

        warp::path("posts").and(
            get_posts
                .or(create_post)
                .or(get_post)
                .or(publish_post)
                .or(unpublish_post),
        )
    };

    println!("Listening incoming connexions...");

    serve(posts_api.recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
