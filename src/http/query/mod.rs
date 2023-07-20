mod class_uid;
mod classes;
mod perms;
use axum::{response::IntoResponse, extract::Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Req {
    pub uid: String,
    pub query: String,
}

pub enum QueryType {
    class_uid,
    get_classes,
    get_class,
    is_teacher,
    new_class(String),
}

impl QueryType {
    pub fn from_string(query: &str) -> Option<QueryType> {
        match query {
            "class_uid" => Some(QueryType::class_uid),
            "get_classes" => Some(QueryType::get_classes),
            "get_class" => Some(QueryType::get_classes),
            "is_teacher" => Some(QueryType::get_classes),
            _ => {
                if query.contains("new_class") {
                    Some(QueryType::new_class(query[9..].to_string()))
                } else {
                    None
                }
            },
        }
    }
}

pub async fn get_response(q: Option<Query<Req>>) -> Option<impl IntoResponse> {
    if let Some(req) = q {
        let uid: &str = req.uid.as_ref();
        if let Some(req) = QueryType::from_string(req.query.as_ref()) {
            Some(match req {
                QueryType::class_uid => class_uid::get(uid).await.into_response(),
                QueryType::get_classes => classes::get(uid).await.into_response(),
                QueryType::get_class => classes::get_one(uid).await.into_response(),
                QueryType::is_teacher => perms::is_teacher(uid).await.into_response(),
                QueryType::new_class(data) => classes::new(uid, data).await.into_response(),
                _ => "400".into_response(),
            })
        } else {
            None
        }
    } else {
        None
    }
}