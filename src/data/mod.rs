pub mod database;

use serde::{Deserialize, Serialize};

// Implement the structure of the User, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub uid: String, // The Unique IDentifier of the user
    pub class_uids: Vec<String>, // The classrooms the user is subscribed to by the their UIDs
}

// Implement the structure of the Classroom, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Classroom {
    pub uid: String, // The Unique IDentifier of the classroom
    pub name: String, // Its plain name, ie. 03DTP-2023
    pub users: Vec<String>, // The list of users UID that are subscribed to the class
}
