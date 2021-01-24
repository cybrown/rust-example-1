mod adapters;
mod application;
mod atomic_counter;
mod db;
mod posts_dao;
mod println_logger;
mod schema;
mod service_registry;
mod simple_counter;
mod uppercaser;

#[macro_use]
extern crate diesel;

use crate::application::Uppercaser;
use crate::service_registry::ServiceRegistry;
use warp::Filter;

#[tokio::main]
async fn main() {
    let sr = ServiceRegistry::new();
    let uppercaser = sr.get_uppercaser();
    let logger: &dyn application::Logger = &sr.get_logger("server".to_owned());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(move |name| {
        logger.log("Incoming request".to_owned());
        format!("Hello, {}!", uppercaser.to_uppercase(name))
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
