mod data;
mod http;

#[allow(unused_imports)]
use data::{Classroom, database::ExtendFirestoreDb};

use crate::{data::database, http::ip_and_port};

// The asynchronous main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Puts all environment variables found in the .env into the environment variables
    dotenv::dotenv().ok();

    println!("Setting up the database.");
    // Create an instance of the database, and unwrap the result (panic if there is an error)
    *database::INSTANCE.lock().await = Some(database::create_firestore_instance().await?);
    

    let fields: [&str; 3] = ["03TEST","Test Teacher","[\"100392388378034835371\"]"];

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

    dbg!(new_class);

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

    // let new_class_uid = Classroom::create_uid();

    // // Add an example classroom into the database
    // {
    //     println!("Seting up new Classroom...");
    //     let instance = database::INSTANCE.lock().await;
    //     let new_classroom = Classroom {
    //         uid: new_class_uid.clone(),
    //         name: String::from("Other-03OTH"),
    //         users: vec!(),
    //         teachers_uids: vec![String::from("100392388378034835371")],
    //         teacher_name: String::from("Jane Doe"),
    //     };

    //     let db = instance.as_ref().unwrap();

    //     println!("Adding new Classroom...");
    //     db.add_class(new_classroom).await.unwrap();

    //     println!("Assigning user and class to each other...");
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