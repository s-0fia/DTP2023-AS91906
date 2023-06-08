use axum::{routing::get, Router};
// use axum_server::tls_rustls::RustlsConfig;
use itertools::Itertools;
use std::{env, net::SocketAddr};
mod shutdown;
mod handlers;

// Initialise const values  for server stuff.
const SERV_PORT: u16 = 80;
// const IP_ADDR: [u8; 4] = [192, 168, 235, 72];
const LOCAL_HOST: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ip_addr: [u8; 4] = if args.len() >= 2 {
        let numbers: Vec<&str> = args[1].split('.').collect();
        if numbers.len() == 4 {
            let mut parsed: Vec<u8> = vec![];
            for num in numbers {
                let n = num.parse::<u8>();
                if n.is_ok() {
                    parsed.push(n.unwrap());
                }
            }
            if parsed.len() == 4 {
                [parsed[0], parsed[1], parsed[2], parsed[3]]
            } else {
                LOCAL_HOST
            }
        }
        else {
            LOCAL_HOST
        }
    } else {
        LOCAL_HOST
    };

    if ip_addr == LOCAL_HOST {
        println!("No (valid) ip address given. Using local host by default!");
        println!("If you wish to use an ip address other than local host, run the exe followed by the ip address. Ie: {} 192.168.x.x", args[0]);
    }

    // Build the routes for the app, handling the static and public directories with an exception for / going to "index.html".
    let app = Router::new()
        .route("/", get(handlers::home_path))
        .route("/*path", get(handlers::public_path))
        .route("/static/*path", get(handlers::static_path));

    // Print the ip and port the server is hosted on.
    println!("Server hosted statically on {}:{} ", ip_addr.iter().format("."), SERV_PORT);
    let access_url = if ip_addr == LOCAL_HOST { String::from("localhost") } else { ip_addr.iter().format(".").to_string() };
    let access_port = if SERV_PORT != 80 { format!(":{}", SERV_PORT) } else { String::from("") };
    println!("Access the GUI by going to {}{}", access_url, access_port);

    // Bind the server with the routes and have a graceful shutdown signal.
    axum::Server::bind(&SocketAddr::from((ip_addr, SERV_PORT)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();
}