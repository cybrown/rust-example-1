mod adapters;
mod application;
mod atomic_counter;
mod db;
mod diesel_post_db;
mod post_controller;
mod println_logger;
mod schema;
mod service_registry;
mod simple_counter;
mod uppercaser;
mod util;

#[macro_use]
extern crate diesel;

use crate::application::Logger;
use crate::application::Uppercaser;
use crate::post_controller::PostController;
use crate::service_registry::ServiceRegistry;
use std::convert::Infallible;
use std::sync::Arc;
use warp::get;
use warp::path;
use warp::post;
use warp::serve;
use warp::{Filter, Rejection, Reply};

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    dbg!(err);
    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

#[tokio::main]
async fn main() {
    let sr = ServiceRegistry::new();
    let uppercaser = Arc::new(sr.get_uppercaser());
    let logger = Arc::new(sr.get_logger("server".to_owned()));
    let post_controller1 = sr.get_post_controller();
    let post_controller2 = sr.get_post_controller();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String).map(move |name| {
        logger.log("Incoming request".to_owned());
        format!("Hello, {}!", uppercaser.to_uppercase(name))
    });

    let get_posts = get()
        .map(move || post_controller1.clone())
        .and_then(PostController::get_posts);

    let create_post = post()
        .map(move || post_controller2.clone())
        .and_then(PostController::create_post);

    let posts_api = path!("posts").and(get_posts.or(create_post));

    println!("Listening incoming connexions...");

    serve(hello.or(posts_api).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
