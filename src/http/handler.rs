use axum::{
    body,
    extract::{Path, Query},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use crate::{http::query::*, data::Req};
use include_dir::{include_dir, Dir};
use mime_guess::mime;

// The directories to include into the binary's compilation (compiled into the .exe)
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
static PUBLIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");
static CLASS_DIR:  Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/class");

// The classroom route handler. Handles all /c/* paths
pub async fn class() -> impl IntoResponse {
    // Get the index.html file from the /class folder
    match CLASS_DIR.get_file("index.html") {
        None => panic!("No index.html found in /class!"), // If none: crash as program is unuable
        Some(file) => Response::builder() // Else build the response with the /class/index.html file
            .status(
                StatusCode::OK
            ).header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime::TEXT_HTML.as_ref()
            ).unwrap(),
            ).body(
                body::boxed(
                    body::Full::from(
                        file.contents()
                    )
                )
            ).unwrap(),
    }
}

// The handler for static paths which is an export for path_handler with the STATIC_DIR
pub async fn static_path(path: Path<String>) -> impl IntoResponse {
    // Pass on to the path handler with the static directory
    path_handler(&STATIC_DIR, path).await
}

// Public path handler with either pass on to path_handler with PUBLIC_DIR or handles queries
pub async fn public_path(path: Path<String>, query: Option<Query<Req>>) -> impl IntoResponse {
    // If the request is a query to the database, then 
    if let Some(res) = handle_query(query).await {
        res.into_response() // Get query and respond with it
    } else { // Else serve the requested html route
        path_handler(&PUBLIC_DIR, path).await.into_response()
    }
}

// Home route handler which exports path_handler with the "/" route
pub async fn home_path(query: Option<Query<Req>>) -> impl IntoResponse {
    public_path(Path(String::from("/")), query).await
}

// Serves all files given by the directories (/public/* and /static/*)
async fn path_handler(directory: &Dir<'static>, Path(path): Path<String>) -> impl IntoResponse {
    // If no file type is specified, add a .html (or index.html for '/' path) at the end...
    // ... so that pages appear as /welcome instead of /welcome.html 
    let path: &str = &{
        let path: String = path.trim_start_matches('/').to_owned();
        if path.is_empty() {
            String::from("index.html")
        } else if path.contains('.') {
            path
        } else {
            path + ".html"
        }
    };

    // Get the file extension type to serve the correct header
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    // Get the file in the directory designated by path
    match directory.get_file(path) {
        // If there is no file at that path, give a code 404 with specific content
        None => Response::builder()
            .status(
                StatusCode::NOT_FOUND
            ).body(
                body::boxed(
                    body::Empty::new()
                )
            ).unwrap(),
        // If there is a file there, serve the file with a code 200
        Some(file) => Response::builder()
            .status(
                StatusCode::OK
            ).header(
                header::CONTENT_TYPE,
                // Convert extension to the correct header type
                HeaderValue::from_str(
                    mime_type.as_ref()
                ).unwrap(),
            ).body(
                body::boxed(
                    body::Full::from(
                        // Serve file content
                        file.contents()
                    )
                )
            ).unwrap(),
    }
}