use crate::api_warp::PostController;
use crate::service_registry::ServiceRegistry;
use domain::Logger;
use domain::Uppercaser;
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
    let mut service_registry = ServiceRegistry::new();
    service_registry.init().await;

    let hello = {
        let uppercaser = Arc::new(service_registry.get_uppercaser());
        let logger = Arc::new(service_registry.get_logger("server".to_owned()));

        path!("hello" / String).map(move |name| {
            logger.log("Incoming request".to_owned());
            format!("Hello, {}!", uppercaser.to_uppercase(name))
        })
    };

    let posts_api = {
        let with_post_controller = {
            let post_controller = service_registry.get_post_controller();
            warp::any().map(move || post_controller.clone())
        };

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

    serve(hello.or(posts_api).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
