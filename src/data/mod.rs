pub mod database;

use serde::{Deserialize, Serialize};

// Implement the enumaration of the Permissions of the user which determines what they can do
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Permissions {
    Admin = -1, // The values that these "map" to, where admin is -1
    Student = 0, // Student is 0
    Teacher = 1, // And Teacher is 1
}

// Implement the structure of the User, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub uid: String, // The Unique IDentifier of the user
    pub class_uids: Vec<String>, // The classrooms the user is subscribed to by the their UIDs
    pub access: Permissions, // The permission level of the user
}

// Implement the structure of the Classroom, which is used by the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Classroom {
    pub uid: String, // The Unique IDentifier of the classroom
    pub name: String, // Its plain name, ie. 03DTP-2023
    pub users: Vec<String>, // The list of users UID that are subscribed to the class
    pub teachers_uids: Vec<String>, // The teachers' UIDs of the class
}
