use axum::response::IntoResponse;

use crate::data::database::{self, ExtendFirestoreDb};

pub async fn get(uid: &str) -> impl IntoResponse {
    let instance = database::INSTANCE.lock().await;
    
    if let Some(db) = instance.as_ref() {
        if let Some(user) = db.find_user_by_id(uid).await {
            serde_json::to_string(&user).unwrap().into_response()
        } else {
            "400".into_response()
        }
    } else {
        "500".into_response()
    }
}