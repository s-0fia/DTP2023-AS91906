use axum::{http::StatusCode, response::IntoResponse};
use crate::data::{*, database::ExtendFirestoreDb};

// Gets Classrooms based an a user uid
pub async fn get(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    // Init the classes vector to be empty
    let mut classes: Vec<ResponseClassroom> = vec![];
    
    // Get the database
    if let Some(db) = instance.as_ref() {
        // Find the user by the uid and get them.
        if let Some(user) = db.find_user_by_id(uid.as_ref()).await {
            let class_uids: Vec<String> = user.class_uids;
            // Iterate of the vector of classes
            for c_uid in class_uids {
                // Find the class
                let class: Option<Classroom> = db.find_class_by_id(&c_uid).await;
                // Push only if the class was found
                if let Some(class) = class {
                    classes.push(class.to_response());
                }
            }
        }
    }

    // Return the serialised vector of classes
    serde_json::to_string(&classes).unwrap().into_response()
}

// Get one classroom based on the class UID
pub async fn get_one(uid: &str) -> impl IntoResponse  {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    // Get the database
    if let Some(db) = instance.as_ref() {
        // Get the classroom and convert it to a ResponseClassroom optional type
        let class: Option<ResponseClassroom> = Classroom::option_to_response(
            db.find_class_by_id(uid).await
        );

        if let Some(class) = class {
            // If the class is found then return it serialised
            serde_json::to_string(&class).unwrap().into_response()
        } else {
            // Otherwise it was a BRE and send that
            StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        // If the db was not found, then return an ISE
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

// Create a new classroom given a valid teacher UID
pub async fn new(uid: &str, q: String) -> impl IntoResponse {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;

    // Get the database
    if let Some(db) = instance.as_ref() {
        // Get the user who sent the request
        if let Some(user) = db.find_user_by_id(uid).await {
            // Check there access level
            match user.access {
                // If they're a teacher or an admin do nothing
                Permissions::Admin => {},
                Permissions::Teacher => {},
                // Otherwise, forbid the user
                _ => {
                    return StatusCode::FORBIDDEN.into_response();
                }
            }
        } else {
            // If no use was found, then it was a bad request
            return StatusCode::BAD_REQUEST.into_response();
        }

        // Convert the fields to an array and error out if the req was bad
        let fields = serde_json::from_str::<[&str; 2]>(&q);
        if fields.is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        // Unwrap the fields to use
        let fields: [&str; 2] = fields.unwrap();

        // Construct the new classroom struct with the correct fields and defaults
        let new_class = Classroom {
            uid: Classroom::create_uid(),
            name: fields[0].to_string(),
            users: vec!(),
            teacher_name: fields[1].to_string(),
            teachers_uids: vec![uid.to_string()],
        };

        // Clone the classroom uid to use later and turn it to a reference
        let class_uid: &str = &new_class.uid.clone();
        
        // Add the classroom uid to the user and exit if an error occured
        if db.add_class_to_user(uid, class_uid).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // Create the new classroom, and backtrack if an error
        if db.add_class(new_class).await.is_err() {
            println!("Class creation failed! Attempting to remove '{class_uid}' from teacher '{uid}'");
            
            // Attempt to remove the erroneous value (a maxmimum of) 3 times
            let max_attempts: usize = 3;
            for i in 1..=max_attempts {
                if db.remove_class_from_user(uid, class_uid).await.is_ok() {
                    println!("Backtrack attempt #{i} of {max_attempts} successful. Class_uids value(s) for teacher restored.");
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                println!("Backtrack attempt #{i} of {max_attempts} failed...");
            }
            
            println!("Backtrack failed! Manual DB intervention necessary.");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // No errors return the OK
        StatusCode::OK.into_response()
    } else {
        // ISE occurred, return as such
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

// Join a new classroom given a valid user UID
pub async fn join(uid: &str, q: String) -> impl IntoResponse {
    // Get the lock on the database to make a query
    let instance = database::INSTANCE.lock().await;
    
    // Get the database
    if let Some(db) = instance.as_ref() {
        // Add the class to the user and return result
        if db.add_class_to_user(uid, &q).await.is_ok() {
            StatusCode::OK.into_response()
        } else {
            StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        // If db didn't work: return ISE
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}