use axum::{
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderValue, header},
    body,
    extract::Path,
};
use include_dir::{include_dir, Dir};

// The directories to include into the binary's compilation (compiled into the .exe)
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
static PUBLIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

// The handler for static paths which is an export for path_handler with the STATIC_DIR
pub async fn static_path(path: Path<String>) -> impl IntoResponse {
    path_handler(&STATIC_DIR, path).await
}

// Export for path_handler with the PUBLIC_DIR
pub async fn public_path(path: Path<String>) -> impl IntoResponse {
    path_handler(&PUBLIC_DIR, path).await
}

// The home path handler "/"
pub async fn home_path() -> impl IntoResponse {
    path_handler(&PUBLIC_DIR, Path(String::from("/"))).await
}

// Serves all files given by the directories (/public/* and /static/*)
async fn path_handler(directory: &Dir<'static>, Path(path): Path<String>) -> impl IntoResponse {
    // If no file type is specified, add a .html (or index.html for '/' path) at the end so that sites appear as /welcome instead of /welcome.html 
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

    // Guess the type of the file from the extension, ie. if it ends with html then it's probably an html file
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    // Get the file in the directory designated by path
    match directory.get_file(path) {
        // If there is no file at that path, give a code 404
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
                    // Use the guessed file type to serve it as that type
                HeaderValue::from_str(mime_type.as_ref())
                    .unwrap(),
            ).body(
                body::boxed(
                    body::Full::from(
                        file.contents()
                    )
                )
            ).unwrap(),
    }
}