use axum::{http::StatusCode, response::IntoResponse};
use crate::data::{*, database::ExtendFirestoreDb};

// Get the class_uids of a given user
pub async fn get(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    // Get the DB
    if let Some(db) = instance.as_ref() {
        // Find the user by the uid and get them.
        if let Some(user) = db.find_user_by_id(uid.as_ref()).await {
            // Return the list of classes to caller based on the user
            return serde_json::to_string(&user.class_uids).unwrap().into_response();
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
    }

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}