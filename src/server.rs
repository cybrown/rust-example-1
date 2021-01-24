mod adapters;
mod application;
mod atomic_counter;
mod db;
mod diesel_post_db;
mod println_logger;
mod schema;
mod service_registry;
mod simple_counter;
mod uppercaser;

#[macro_use]
extern crate diesel;

use crate::application::Logger;
use crate::application::PostDb;
use crate::application::Uppercaser;
use crate::service_registry::ServiceRegistry;
use warp::Filter;

#[tokio::main]
async fn main() {
    let sr = ServiceRegistry::new();
    let uppercaser = sr.get_uppercaser();
    let logger = sr.get_logger("server".to_owned());
    let post_db = sr.get_post_db();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(move |name| {
        logger.log("Incoming request".to_owned());
        format!("Hello, {}!", uppercaser.to_uppercase(name))
    });

    let get_posts = warp::path!("posts").map(move || {
        post_db
            .get_posts()
            .map(|posts| warp::reply::json(&posts))
            .unwrap()
    });

    warp::serve(hello.or(get_posts))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
