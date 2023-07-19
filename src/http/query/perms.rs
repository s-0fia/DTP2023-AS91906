use axum::response::IntoResponse;

use crate::data::{database::{self, ExtendFirestoreDb}, Permissions};

pub async fn is_teacher(uid: &str) -> impl IntoResponse {
    serde_json::to_string(
        &check_perms(
            uid,
            Permissions::Teacher
        ).await
    ).unwrap()
    .into_response()
}

pub async fn is_student(uid: &str) -> impl IntoResponse {
    serde_json::to_string(
        &check_perms(uid,
            Permissions::Student
        ).await
    ).unwrap()
    .into_response()
}

pub async fn is_admin(uid: &str) -> impl IntoResponse {
    serde_json::to_string(
        &check_perms(uid,
            Permissions::Admin
        ).await
    ).unwrap()
    .into_response()
}

async fn check_perms(uid: &str, is: Permissions) -> bool {
    let instance = database::INSTANCE.lock().await;
    
    let mut result: bool = false;

    if let Some(db) = instance.as_ref() {
        if let Some(user) = db.find_user_by_id(uid).await {
            result = user.access == is;
        }
    }

    result
}