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
                    return "403".into_response();
                }
            }
        }

        let fields = serde_json::from_str::<[&str; 3]>(&q);
        
        let fields = if fields.is_err() {
            return "400".into_response();
        } else {
            fields.unwrap()
        };

        let teachers_uids: Vec<String> = 
            if let Ok(uids) = serde_json::from_str(fields[2]) {
                uids
            } else {
                vec!()
            };

        let new_class = Classroom {
            uid: Classroom::create_uid(),
            name: fields[0].to_string(),
            users: vec!(),
            teacher_name: fields[1].to_string(),
            teachers_uids,
        };

        if db.add_class(new_class).await.is_ok() {
            "200".into_response()
        } else {
            "400".into_response()
        }
    } else {
        "500".into_response()
    }
}