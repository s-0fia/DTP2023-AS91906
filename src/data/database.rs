use async_trait::async_trait;
use crate::data::*;
use firestore::{FirestoreDb, errors::FirestoreError, FirestoreDbOptions};
use gcloud_sdk::{GCP_DEFAULT_SCOPES, TokenSourceType};
use std::env::var;

// Trait which extends the FirebaseDb object so that extension methods can be used
// ie. db.add_user(new_user_info) instead of 
#[async_trait]
pub trait ExtendFirestoreDb {
    // * User data relating database functions
    // Finds a user in the database by their uid and if the user exsists then it returns it, otherwise it returns None.
    async fn find_user_by_id(&self, uid: &str) -> Option<User>;
    // Adds a user to the database, and passes the error back to the caller if an error is raised.
    async fn add_user(&self, user: User) -> Result<(), FirestoreError>;
    // Deletes a user from the database, and passes the error back to the caller if an error is raised.
    async fn remove_user(&self, uid: &str) -> Result<(), FirestoreError>;

    // * Classroom data relating database functions
    // Finds a user in the database by their uid and if the user exsists then it returns it, otherwise it returns None.
    async fn find_class_by_id(&self, uid: &str) -> Option<Classroom>;
    // Adds a user to the database, and passes the error back to the caller if an error is raised.
    async fn add_class(&self, class: Classroom) -> Result<(), FirestoreError>;
    // Deletes a user from the database, and passes the error back to the caller if an error is raised.
    async fn remove_class(&self, uid: &str) -> Result<(), FirestoreError>;
}

// Creates an instance of a firestore database to be used to interact with the database.
pub async fn create_firestore_instance() -> Result<FirestoreDb, FirestoreError> {
    // Get the project id from the .env file, or stops the program with a descriptive error detailing how
    // to set or change the project_id
    let project_id = var("PROJECT_ID")
        .expect("No project ID found in the .env!\n\
                Please add the field PROJECT_ID=edsyca where the project id is the public alias of the project in the .env."
            );

    // Gets the path to the google key file, or stops the program with a descriptive error detailing how
    // to change it and how to generate the key.
    let google_key_path = var("GOOGLE_KEY_PATH")
        .expect("No path to google key found in the .env.\n\
                Please add the field GOOGLE_KEY_PATH= followed by a valid path to the .json file containing the google api key.\n\
                For more information please go to: https://cloud.google.com/iam/docs/keys-create-delete#creating."
            ); 

    // Creates the firestore database with the project_id and with the google key
    FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(project_id.to_string()),
        GCP_DEFAULT_SCOPES.clone(),
        TokenSourceType::File(google_key_path.into())
    ).await
}

// Implements the functions defined by the trait
#[async_trait]
impl ExtendFirestoreDb for FirestoreDb {
    async fn find_user_by_id(&self, uid: &str) -> Option<User> {
        self.fluent()           // Converts the database (self) to a useable format
            .select()           // Sets the mod to selecting data from it
            .by_id_in("users")  // Data in the "users" collection
            .obj()              // Get an object from this
            .one(uid)          // Get the (one) object with the given uid
            .await
            .unwrap()
    }
    
    async fn add_user(&self, user: User) -> Result<(), FirestoreError> {
        // Check if a user already exists with this uid
        if self.find_user_by_id(&user.uid).await.is_none() {
            self.fluent()               // Converts the database (self) to a useable format
                .insert()               // Sets it to insert mode
                .into("users")          // Into the user collection
                .document_id(&user.uid) // With the id of user.uid
                .object(&user)          // Insert the whole object
                .execute()              // Execute the command
                .await
        } else {
            // If the user is already in the database return Ok
            Ok(())
        }
    }

    async fn remove_user(&self, uid: &str) -> Result<(), FirestoreError> {
        // Check if a user exists with this uid
        if self.find_user_by_id(uid).await.is_some() {
            self.fluent()           // Converts the database (self) to a useable format
                .delete()           // Set to to delete mode
                .from("users")      // From the users collection
                .document_id(uid)   // Remove the data id'd by uid
                .execute()          // Execute the command
                .await
        } else {
            // If the user is not already in the database return Ok
            Ok(())
        }
    }

    async fn find_class_by_id(&self, uid: &str) -> Option<Classroom> {
        self.fluent()            // Converts the database (self) to a useable format
            .select()            // Sets the mod to selecting data from it
            .by_id_in("clasroom")// Data in the "classroom" collection
            .obj()               // Get an object from this
            .one(uid)            // Get the (one) classroom object with the given uid
            .await
            .unwrap()
    }

    async fn add_class(&self, class: Classroom) -> Result<(), FirestoreError> {
        // Check if a classroom already exists with this uid
        if self.find_class_by_id(&class.uid).await.is_none() {
            self.fluent()               // Converts the database (self) to a useable format
                .insert()               // Sets it to insert mode
                .into("classroom")      // Into the classroom collection
                .document_id(&class.uid)// With the id of class.uid
                .object(&class)         // Insert the whole object
                .execute()              // Execute the command
                .await
        } else {
            // If the classroom is already in the database return Ok
            Ok(())
        }
    }

    async fn remove_class(&self, uid: &str) -> Result<(), FirestoreError> {
        // Check if a classroom exists with this uid
        if self.find_user_by_id(uid).await.is_some() {
            self.fluent()           // Converts the database (self) to a useable format
                .delete()           // Set to to delete mode
                .from("classroom")  // From the clasroom collection
                .document_id(uid)   // Remove the data id'd by uid
                .execute()          // Execute the command
                .await
        } else {
            // If the user is not already in the database return Ok
            Ok(())
        }
    }
}