use axum::response::IntoResponse;
use crate::data::{*, database::ExtendFirestoreDb};

pub async fn get(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    if let Some(db) = instance.as_ref() {
        // Find the user by the uid and get them.
        if let Some(user) = db.find_user_by_id(uid.as_ref()).await {
            dbg!(&user.class_uids);
            // Return the list of classes to caller based on the user
            return serde_json::to_string(&user.class_uids).unwrap().into_response();
        } else {
            println!("Added new (student) user into database!");

            let new_user = User {
                uid: uid.to_string(),
                class_uids: vec!(),
                access: Permissions::Student,
            };
            
            db.add_user(new_user).await.unwrap();
        }
    }

    let no_classes: Vec<String> = vec!();
    serde_json::to_string(&no_classes).unwrap().into_response()
}