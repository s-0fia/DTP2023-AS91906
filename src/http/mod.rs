mod handler;
pub mod ip_and_port;
mod shutdown;

use axum::{routing::get, Router};
use std::{net::SocketAddr, alloc::handle_alloc_error};

pub async fn create_server(ip_addr: [u8; 4], port: u16) {
    // Build the routes for the app, handling the static and public directories with an exception for / going to "index.html".
    let app = Router::new()
        .route("/get/*path", get(handler::get_request))
        .route("/", get(handler::home_path))
        .route("/*path", get(handler::public_path))
        .route("/static/*path", get(handler::static_path));

    // Bind the server with the routes and have a graceful shutdown signal.
    axum::Server::bind(&SocketAddr::from((ip_addr, port)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}