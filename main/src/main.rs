mod configuration;
mod service_registry;

use crate::service_registry::ServiceRegistry;
use api::run_server;

async fn run_create_post_command(sr: ServiceRegistry) {
    let create_post_command = sr.get_create_post_command();
    create_post_command.run().await.expect("failed");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let mut sr = ServiceRegistry::new();
        sr.init().await;
        if args[1] == "server" {
            run_server(Box::new(sr.get_post_domain())).await;
        } else if args[1] == "create_post" {
            run_create_post_command(sr).await;
        }
    }
}
