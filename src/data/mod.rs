pub mod database;

use rand::Rng;
use serde::{Deserialize, Serialize};

//* DATABASE RELATED DATA STRUCTURES AND IMPLEMENTATIONS

// The structure of the User, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub uid: String, // The Unique IDentifier of the user
    pub class_uids: Vec<String>, // The classrooms the user is subscribed to by the their UIDs
    pub access: Permissions, // The permission level of the user
}

// The structure of the Classroom, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Classroom {
    pub uid: String, // The Unique IDentifier of the classroom
    pub name: String, // Its plain name, ie. 03DTP-2023
    pub users: Vec<String>, // The list of users UID that are subscribed to the class
    pub teachers_uids: Vec<String>, // The teachers' UIDs of the class
    pub teacher_name: String, // The name of the primary teacher(s) who owns/runs the class
}

// Implement the classroom related methods
impl Classroom {
    // Creates a uid for a classroom
    pub fn create_uid() -> String {
        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();
        (0..10).map(|_|
            digits.chars()
                .nth(
                    rng.gen_range(0..36)
                ).unwrap()
        ).collect()
    }

    // Converts a Classroom type to a ResponseClassroom type for public use
    pub fn to_response(&self) -> ResponseClassroom {
        ResponseClassroom {
            uid: self.uid.clone(),
            name: self.name.clone(),
            teachers_uids: self.teachers_uids.clone(),
            teacher_name: self.teacher_name.clone(),
        }
    }

    // Converts an Option<Classroom> an Option<ResponseClassroom> for public use
    pub fn option_to_response(class: Option<Self>) -> Option<ResponseClassroom> {
        class.map(|class| class.to_response())
    }
}

// The structure of ResponseClassroom, a public restricted export of Classroom, used for frontend
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseClassroom {
    pub uid: String, // The Unique IDentifier of the classroom
    pub name: String, // Its plain name, ie. 03DTP-2023
    pub teachers_uids: Vec<String>, // The teachers' UIDs of the class
    pub teacher_name: String, // The name of the primary teacher(s) who own/run the class
}

//* QUERY RELATED DATA STRUCTURES AND IMPLEMENTATIONS

// The query variants
pub enum QueryType {
    AddUser,
    AddClass(String),
    ClassUid,
    GetClasses,
    GetClass,
    GetUser,
    NewClass(String),
}

// The permission variants, to determine a user's access level
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Permissions {
    Admin = -1, // The values that these "map" to, where admin is -1
    Student = 0, // Student is 0
    Teacher = 1, // And Teacher is 1
}

// The structure of the api requests, eg. ".../dashboard?uid=3457883495804+q=class_uid"
#[derive(Debug, Deserialize)]
pub struct Req {
    pub uid: String, // The user/class uid for the request
    pub q: String, // The query parameters / load
}

// Impletementing an extension function for the types of queries
impl QueryType {
    // Converts from &str to a variant of QueryType or none
    pub fn from_string(query: &str) -> Option<QueryType> {
        Some(match query {
            "add_user" => QueryType::AddUser,
            "class_uid" => QueryType::ClassUid,
            "get_classes" => QueryType::GetClasses,
            "get_class" => QueryType::GetClass,
            "get_user" => QueryType::GetUser,
            _ => { // Default path
                // If it is a new_class* query
                if query.contains("new_class") {
                    // Return a NewClass variant with data attached
                    QueryType::NewClass(
                        // Substring the query to only give whats after new_class
                        query[9..].to_string()
                    )
                } else if query.contains("add_class") {
                    QueryType::AddClass(
                        query[9..].to_string()
                    )
                } else {
                    // If no match return None instead of Some(variant)
                    return None;
                }
            },
        })
    }
}