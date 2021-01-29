use crate::application::Logger;
use crate::application::Uppercaser;
use crate::post_controller::PostController;
use crate::service_registry::ServiceRegistry;
use std::convert::Infallible;
use std::sync::Arc;
use warp::get;
use warp::path;
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

pub async fn run_server() {
    let sr = ServiceRegistry::new();
    let uppercaser = Arc::new(sr.get_uppercaser());
    let logger = Arc::new(sr.get_logger("server".to_owned()));

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String).map(move |name| {
        logger.log("Incoming request".to_owned());
        format!("Hello, {}!", uppercaser.to_uppercase(name))
    });

    let post_controller = sr.get_post_controller();
    let with_post_controller = warp::any().map(move || post_controller.clone());

    let get_posts = with_post_controller
        .clone()
        .and(get())
        .and(warp::path::end())
        .and(warp::query())
        .and_then(PostController::get_posts);

    let create_post = with_post_controller
        .clone()
        .and(post())
        .and(warp::path::end())
        .and_then(PostController::create_post);

    let publish_post = with_post_controller
        .and(put())
        .and(warp::path!(i32 / "published"))
        .and_then(PostController::publish_post);

    let posts_api = warp::path("posts").and(get_posts.or(create_post).or(publish_post));

    println!("Listening incoming connexions...");

    serve(hello.or(posts_api).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
