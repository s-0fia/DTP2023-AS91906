use axum::{response::IntoResponse, http::StatusCode};
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

pub async fn get_one(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;
    
    let class: Option<ResponseClassroom> = if let Some(db) = instance.as_ref() {
        Classroom::option_to_response(
            db.find_class_by_id(uid)
                .await
        )
    } else {
        None
    };

    if let Some(class) = class {
        serde_json::to_string(&class).unwrap().into_response()
    } else {
        "400".into_response()
    }
}

pub async fn new(uid: &str, q: String) -> impl IntoResponse {
    let instance = database::INSTANCE.lock().await;

    if let Some(db) = instance.as_ref() {
        if let Some(user) = db.find_user_by_id(uid).await {
            match user.access {
                Permissions::Admin => {},
                Permissions::Teacher => {},
                _ => {
                    return StatusCode::FORBIDDEN.into_response();
                }
            }
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }

        let fields = serde_json::from_str::<[&str; 2]>(&q);
        
        if fields.is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }

        let fields = fields.unwrap();

        let new_class = Classroom {
            uid: Classroom::create_uid(),
            name: fields[0].to_string(),
            users: vec!(),
            teacher_name: fields[1].to_string(),
            teachers_uids: vec![uid.to_string()],
        };

        let class_uid = &new_class.uid.clone();
        
        if db.add_class_to_user(uid, class_uid).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if db.add_class(new_class).await.is_err() {
            todo!("Remove class from user");
            return StatusCode::BAD_REQUEST.into_response();
        }

        StatusCode::OK.into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}