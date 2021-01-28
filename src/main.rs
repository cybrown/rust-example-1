mod adapters;
mod application;
mod atomic_counter;
mod db;
mod diesel_post_db;
mod post_controller;
mod println_logger;
mod schema;
mod server;
mod service_registry;
mod simple_counter;
mod uppercaser;
mod util;

#[macro_use]
extern crate diesel;

use crate::server::run_server;
use crate::service_registry::ServiceRegistry;

fn dummy_cli_command() {
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "server" {
            run_server().await;
        } else if args[1] == "dummy" {
            dummy_cli_command();
        }
    }
}
