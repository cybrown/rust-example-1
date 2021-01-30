use crate::PostController;
use std::convert::Infallible;
use warp::get;
use warp::post;
use warp::put;
use warp::serve;
use warp::{Filter, Rejection, Reply};

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
            .and(warp::path::end())
            .and_then(PostController::create_post);

        let publish_post = put()
            .and(with_post_controller)
            .and(warp::path!(i32 / "published"))
            .and_then(PostController::publish_post);

        warp::path("posts").and(get_posts.or(create_post).or(publish_post))
    };

    println!("Listening incoming connexions...");

    serve(posts_api.recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
