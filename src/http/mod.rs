mod handler;
pub mod ip_and_port;
mod query;
mod shutdown;

use axum::{Router, routing::get};
use std::net::SocketAddr;

// Builds the axum.rs server and routes it to the port and ip address given
pub async fn create_server(ip_addr: [u8; 4], port: u16) {
    // Build the routes for the app, handling the static and public directories with an exception for / going to "index.html".
    let app = Router::new()
        .route("/", get(handler::home_path)) // The / path to go to index.html
        .route("/*path", get(handler::public_path)) // The public path, eg. "/dashboard"
        .route("/static/*path", get(handler::static_path)) // The static path for static content, eg. js, css, etc.
        .route("/c/*path", get(handler::class)); // The classrooms path, for individual classroom access

    // Bind the server with the routes and have a graceful shutdown signal.
    axum::Server::bind(&SocketAddr::from((ip_addr, port)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}