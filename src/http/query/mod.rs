mod class_uid;
mod classes;
mod user;
use axum::{response::IntoResponse, extract::Query};
use serde::Deserialize;
use crate::data::*;

#[derive(Debug, Deserialize)]
pub struct Req {
    pub uid: String,
    pub q: String,
}

#[allow(unreachable_patterns)]
pub async fn get_response(q: Option<Query<Req>>) -> Option<impl IntoResponse> {
    if let Some(req) = q {
        let uid: &str = req.uid.as_ref();
        if let Some(req) = QueryType::from_string(req.q.as_ref()) {
            Some(match req {
                QueryType::ClassUid => class_uid::get(uid).await.into_response(),
                QueryType::GetClasses => classes::get(uid).await.into_response(),
                QueryType::GetClass => classes::get_one(uid).await.into_response(),
                QueryType::GetUser => user::get(uid).await.into_response(),
                QueryType::NewClass(data) => classes::new(uid, data).await.into_response(),
                _ => "400".into_response(),
            })
        } else {
            None
        }
    } else {
        None
    }
}