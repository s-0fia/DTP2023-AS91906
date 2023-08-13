mod classes;
mod class_uid;
mod user;

use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use crate::data::*;

#[allow(unreachable_patterns)]
pub async fn handle_query(q: Option<Query<Req>>) -> Option<impl IntoResponse> {
    // If the request follows the Req struct pattern
    if let Some(req) = q {
        // Get the uid from the reuqest (as a ref)
        let uid: &str = req.uid.as_ref();
        // If the request is a valid query
        if let Some(req) = QueryType::from_string(req.q.as_ref()) {
            // Process the validated request and pass the responses onwards
            return Some(match req {
                QueryType::AddUser
                    => user::add(uid).await.into_response(),
                QueryType::AddClass(data)
                    => classes::join(uid, data).await.into_response(),
                QueryType::ClassUid
                    => class_uid::get(uid).await.into_response(),
                QueryType::GetClasses
                    => classes::get(uid).await.into_response(),
                QueryType::GetClass
                    => classes::get_one(uid).await.into_response(),
                QueryType::GetUser
                    => user::get(uid).await.into_response(),
                QueryType::NewClass(data)
                    => classes::new(uid, data).await.into_response(),
                _ => StatusCode::BAD_REQUEST.into_response(),
            });
        } else {
            // If the query field existed but was bad, eg. "/uid=567567568+q=" or "/uid=438745869+q=get_calsses"
            return Some(StatusCode::BAD_REQUEST.into_response());
        }
    }
    None // If the request is not valid and therefore not a request
}