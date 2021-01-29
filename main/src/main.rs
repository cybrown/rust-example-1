mod adapters;
mod api_warp;
mod commands;
mod db_diesel;
mod service_registry;
mod util;

#[macro_use]
extern crate diesel;

use crate::api_warp::run_server;
use crate::service_registry::ServiceRegistry;

async fn run_dummy_command() {
    let sr = ServiceRegistry::new();

    // Instantiate many applications who share the same dependencies
    let dummy_command1 = sr.get_dummy_command();
    let dummy_command2 = sr.get_dummy_command();

    // Run the applications with the same shared dependencies
    dummy_command1.run().await;
    dummy_command2.run().await;

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
            run_dummy_command().await;
        }
    }
}
