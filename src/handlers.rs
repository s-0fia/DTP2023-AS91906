use axum::{
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderValue, header},
    body,
    extract::Path,
};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
static PUBLIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

pub async fn static_path(path: Path<String>) -> impl IntoResponse {
    path_handler(&STATIC_DIR, path).await
}

pub async fn public_path(path: Path<String>) -> impl IntoResponse {
    path_handler(&PUBLIC_DIR, path).await
}

pub async fn home_path() -> impl IntoResponse {
    path_handler(&PUBLIC_DIR, Path(String::from("/"))).await
}

async fn path_handler(directory: &Dir<'static>, Path(path): Path<String>) -> impl IntoResponse {
    let path: &str = &{
        let path: String = path.trim_start_matches('/').to_owned();
        if path.is_empty() {
            String::from("index.html")
        } else if path.contains(".") {
            path
        } else {
            path + ".html"
        }
    };
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match directory.get_file(path) {
        None => Response::builder()
                .status(
                    StatusCode::NOT_FOUND
                ).body(
                    body::boxed(
                        body::Empty::new()
                    )
                ).unwrap(),
        Some(file) => Response::builder()
                .status(
                    StatusCode::OK
                ).header(
                    header::CONTENT_TYPE,
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