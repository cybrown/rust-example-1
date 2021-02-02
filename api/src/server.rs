use crate::PostController;
use std::convert::Infallible;
use warp::{
    any, body::json, delete, get, hyper::StatusCode, path, path::end, post, put, query, serve,
    Filter, Rejection, Reply,
};

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    dbg!(err);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn run_server(post_controller: PostController) {
    let posts_api = {
        let with_post_controller = any().map(move || post_controller.clone());

        let get_posts = get()
            .and(with_post_controller.clone())
            .and(end())
            .and(query())
            .and_then(PostController::get_posts);

        let create_post = post()
            .and(with_post_controller.clone())
            .and(json())
            .and(end())
            .and_then(PostController::create_post);

        let get_post = get()
            .and(with_post_controller.clone())
            .and(path!(i32))
            .and_then(PostController::get_post);

        let publish_post = put()
            .and(with_post_controller.clone())
            .and(path!(i32 / "published"))
            .and_then(PostController::publish_post);

        let unpublish_post = delete()
            .and(with_post_controller.clone())
            .and(path!(i32 / "published"))
            .and_then(PostController::unpublish_post);

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
