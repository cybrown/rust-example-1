mod adapters;
mod service_registry;

use crate::service_registry::ServiceRegistry;
use api::run_server;

async fn run_dummy_command(sr: ServiceRegistry) {
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
        let mut sr = ServiceRegistry::new();
        sr.init().await;
        if args[1] == "server" {
            run_server(sr.get_post_controller()).await;
        } else if args[1] == "dummy" {
            run_dummy_command(sr).await;
        }
    }
}
