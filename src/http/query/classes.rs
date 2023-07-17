use axum::response::IntoResponse;
use crate::data::{*, database::ExtendFirestoreDb};

pub async fn get(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    // Init the classes vector to be empty
    let mut classes: Vec<ResponseClassroom> = vec![];
    
    // Get the database
    if let Some(db) = instance.as_ref() {
        // Find the user by the uid and get them.
        if let Some(user) = db.find_user_by_id(uid.as_ref()).await {
            let class_uids = user.class_uids;
            for c_uid in class_uids {
                let class: Option<Classroom> = db.find_class_by_id(&c_uid).await;
                if let Some(class) = class {
                    classes.push(class.to_response());
                }
            }
        }
    }
    serde_json::to_string(&classes).unwrap().into_response()
}