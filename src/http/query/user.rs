use axum::{http::StatusCode, response::IntoResponse};
use crate::data::{database::{ExtendFirestoreDb, self}, Permissions, User};

// Get the user data based on the query's uid
pub async fn get(uid: &str) -> impl IntoResponse {
    // Wait for database instance lock
    let instance = database::INSTANCE.lock().await;
    
    // If database is successfully captured
    if let Some(db) = instance.as_ref() {
        // If a user is successfully found
        if let Some(user) = db.find_user_by_id(uid).await {
            // Serialise the user data and respond with it
            serde_json::to_string(&user).unwrap().into_response()
        } else {
            // Else return 400 code
            StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        // Else return 500 code
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn add(uid: &str) -> impl IntoResponse {
    // wait for db instance lock
    let instance = database::INSTANCE.lock().await;
    
    // If db is captured
    if let Some(db) = instance.as_ref() {
        // OK out if user already exists
        if db.find_user_by_id(uid).await.is_some() {
            return StatusCode::OK.into_response();
        }

        // Make the new user with defaults and the given UID
        let new_user = User {
            uid: uid.to_string(),
            class_uids: vec!(),
            access: Permissions::Student,
        };

        // Try to make the new user 3 times before quitting
        let max_attempts: usize = 3;
        for i in 1..=max_attempts {
            // Exit if user addition failed
            if db.add_user(new_user.clone()).await.is_ok() {
                return StatusCode::OK.into_response();
            }
            println!("Adding new user {uid} attempt #{i} of {max_attempts} failed!");
        }

        println!("Adding new user failed! Manual addition necessary.");
    }

    // Send ISE if db was not captured or if user addition failed
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}