mod data;
mod http;

use data::database::INSTANCE;

use crate::{data::database, http::ip_and_port};

// The asynchronous main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Puts all environment variables found in the .env into the environment variables
    dotenv::dotenv().ok();

    // Create an instance of the database, and unwrap the result (panic if there is an error)
    *database::INSTANCE.lock().await = Some(database::create_firestore_instance().await?);
    
    // Get the ip and port defined by the user or default to ip 127.0.0.1 and/or port 80
    let (ip_addr, port) = ip_and_port::get_or_default();
    
    // Print information about: the ip and port, website, and environment variables to the user
    ip_and_port::print_info(&ip_addr, &port);
    
    // Spawn a tokio thread to handle the tcp server
    // tokio::spawn(async move {
    // });

    // Create and host the server (found in /http/mod.rs)
    http::create_server(ip_addr, port).await;

    Ok(())
}