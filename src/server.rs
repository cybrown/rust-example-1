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
mod utils;

#[macro_use]
extern crate diesel;

use crate::application::Logger;
use crate::application::Uppercaser;
use crate::post_controller::PostController;
use crate::service_registry::ServiceRegistry;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    let sr = ServiceRegistry::new();
    let uppercaser = Arc::new(sr.get_uppercaser());
    let logger = Arc::new(sr.get_logger("server".to_owned()));
    let post_controller1 = sr.get_post_controller();
    let post_controller2 = sr.get_post_controller();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(move |name| {
        logger.log("Incoming request".to_owned());
        format!("Hello, {}!", uppercaser.to_uppercase(name))
    });

    let get_posts = warp::path!("posts")
        .map(move || post_controller1.clone())
        .and(warp::filters::method::get())
        .and_then(PostController::get_posts);

    let create_post = warp::path!("posts")
        .map(move || post_controller2.clone())
        .and(warp::filters::method::post())
        .and_then(PostController::create_post);

    warp::serve(hello.or(get_posts).or(create_post))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
