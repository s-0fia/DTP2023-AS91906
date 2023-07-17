mod data;
mod http;

use crate::{data::database::{self, *}, http::{ip_and_port}};

// The asynchronous main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Puts all environment variables found in the .env into the environment variables
    dotenv::dotenv().ok();

    // Create an instance of the database, and unwrap the result (panic if there is an error)
    *database::INSTANCE.lock().await = Some(database::create_firestore_instance().await?);
    
    // let new_user_uid = String::from("105111092662728170806");
    // // Add an example student into the database
    // {
    //     let instance = database::INSTANCE.lock().await;
    //     let new_user = data::User {
    //         uid: new_user_uid.clone(),
    //         class_uids: vec!(),
    //         access: data::Permissions::Student,
    //     };

    //     let db = instance.as_ref().unwrap();
    
    //     db.remove_user(&new_user.uid).await.unwrap();
    //     db.add_user(new_user).await.unwrap();
    // }

    // let new_class_uid = data::Classroom::create_uid();

    // // Add an example classroom into the database
    // {
    //     let instance = database::INSTANCE.lock().await;
    //     let new_classroom = data::Classroom {
    //         uid: new_class_uid.clone(),
    //         name: String::from("Example-03EXC"),
    //         users: vec!(),
    //         teachers_uids: vec!(),
    //     };

    //     let db = instance.as_ref().unwrap();

    //     db.add_class(new_classroom).await.unwrap();

    //     db.assign_class_and_user(&new_user_uid, &new_class_uid).await.unwrap();
    // }
    
    // Get the ip and port defined by the user or default to ip 127.0.0.1 and/or port 80
    let (ip_addr, port) = ip_and_port::get_or_default();
    
    // Print information about: the ip and port, website, and environment variables to the user
    ip_and_port::print_info(&ip_addr, &port);
    
    // Spawn a tokio thread to handle a tcp server
    // tokio::spawn(async move {
    //     tcp::create_server(ip_addr, 3000).await;
    // });

    // Create and host the server (found in /http/mod.rs)
    http::create_server(ip_addr, port).await;

    Ok(())
}