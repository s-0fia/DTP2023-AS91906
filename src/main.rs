use axum::{response::Html, routing::get, Router};
use itertools::Itertools;
use std::net::SocketAddr;
mod shutdown;

// Initialise const values for server stuff.
const SERV_PORT: u16 = 80;
const IP_ADDR: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() {
    // Build the app with the '/' route forwarding to the handler() function.
    let app = Router::new().route("/", get(handler));

    // Print the ip and port the server is hosted on, then bind and serve the server.
    println!("listening on {}:{}", IP_ADDR.iter().format("."), SERV_PORT);
    axum::Server::bind(&SocketAddr::from((IP_ADDR, SERV_PORT)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}

// A simple rHTML function that shows as a default page.
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}