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

use crate::service_registry::ServiceRegistry;

fn main() {
    let sr = ServiceRegistry::new();

    // Instantiate many applications who share the same dependencies
    let app1 = sr.get_application();
    let app2 = sr.get_application();

    // Run the applications with the same shared dependencies
    app1.run();
    app2.run();

    // Show how many time an app was run
    println!("Count: {}", sr.get_counter().get_value());
}
