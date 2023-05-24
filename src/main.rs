use axum::{routing::get, Router};
use itertools::Itertools;
use std::net::SocketAddr;
mod shutdown;
mod handlers;

// Initialise const values for server stuff.
const SERV_PORT: u16 = 80;
const IP_ADDR: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() {
    // Build the routes for the app, handling the static and public directories with an exception for / going to "index.html".
    let app = Router::new()
        .route("/", get(handlers::home_path))
        .route("/*path", get(handlers::public_path))
        .route("/static/*path", get(handlers::static_path));

    // Print the ip and port the server is hosted on.
    println!("Server hosted statically on {}:{}", IP_ADDR.iter().format("."), SERV_PORT);

    // Bind the server with the routes and have a graceful shutdown signal.
    axum::Server::bind(&SocketAddr::from((IP_ADDR, SERV_PORT)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}