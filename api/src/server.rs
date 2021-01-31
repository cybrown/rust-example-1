use crate::PostController;
use std::convert::Infallible;
use warp::{delete, get, post, put, serve, Filter, Rejection, Reply};

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    dbg!(err);
    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn run_server(post_controller: PostController) {
    let posts_api = {
        let with_post_controller = { warp::any().map(move || post_controller.clone()) };

        let get_posts = get()
            .and(with_post_controller.clone())
            .and(warp::path::end())
            .and(warp::query())
            .and_then(PostController::get_posts);

        let create_post = post()
            .and(with_post_controller.clone())
            .and(warp::body::json())
            .and(warp::path::end())
            .and_then(PostController::create_post);

        let get_post = get()
            .and(with_post_controller.clone())
            .and(warp::path!(i32))
            .and_then(PostController::get_post);

        let publish_post = put()
            .and(with_post_controller.clone())
            .and(warp::path!(i32 / "published"))
            .and_then(PostController::publish_post);

        let unpublish_post = delete()
            .and(with_post_controller.clone())
            .and(warp::path!(i32 / "published"))
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
