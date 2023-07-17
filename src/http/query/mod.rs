mod class_uid;
mod classes;
use axum::{response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Req {
    uid: String,
    q: String,
}

pub async fn get_response(q: Option<Query<Req>>) -> Option<impl IntoResponse> {
    if let Some(query) = q {
        
        let user_uid: &str = query.uid.as_ref();
        Some(match query.q.as_ref() {
            "class_uid" => {
                class_uid::get(user_uid).await.into_response()
            },
            "get_classes" => {
                classes::get(user_uid).await.into_response()
            },
            _ => {
                "400".into_response()
            },
        })
    } else {
        None
    }
}