use std::{env::{var, VarError}};
use itertools::Itertools;

// Set the (constant) default sever port of 80
const DEFAULT_PORT: u16 = 80;
// Set the (constant) local host to 127.0.0.1 (for readability)
const LOCAL_HOST: [u8; 4] = [127, 0, 0, 1];

// Gets the ip address from the environment variable and parses it into an array
fn get_env_ip_addr() -> Option<[u8; 4]> {
    // Get value from the enviroment variable (.env) IP as a string
    let ip_str: Result<String, VarError> = var("IP");

    // If the var was found, convert from Result<T, E> to T
    if let Ok(ip_str) = ip_str {
        let parsed: Vec<u8> = ip_str
            .split('.') // Split the string by .
            .filter_map(|n| n.parse::<u8>().ok()) // Parse each part of the string and only keep it if was able to be parsed
            .collect(); // Collect all the successfully parsed parts into a vector

        // If only 4 parts were successfully parsed (as IPs only have 4 numbers 192.168.x.x)
        if parsed.len() == 4 {
            // Return the elements in the form of an Some(array)
            return Some([parsed[0], parsed[1], parsed[2], parsed[3]]);
        }
    }

    // If the return was not reached return None
    None
}

// Gets the port from the environment variables and parses it as a number
fn get_env_port() -> Option<u16> {
    // Get value from the enviroment variable (.env) PORT as a string
    let port: Result<String, VarError> = var("PORT");

    port.unwrap_or(String::from("")) // Converts from an result to a string, and if it was an error, give ""
        .parse::<u16>() // Parse the string into a u16
        .ok() // And convert the result fromt he parse to an Option type, so give either Some(u16) or None
}

// Gets the ip and port from .env or set it to defaults
pub fn get_or_default() -> ([u8; 4], u16) {
    // Get the ip address from the env. vars.    
    let ip_addr: Option<[u8; 4]> = get_env_ip_addr();
    // Relay that no (valid) ip address was given and that local host is being used intead
    if ip_addr.is_none() {
        println!("No (valid) ip address given. Using local host by default!");
    }
    // Get the user defined ip address or use local host instead
    let ip_addr: [u8; 4] = ip_addr.unwrap_or(LOCAL_HOST);

    // Get the port from the env. vars.
    let port: Option<u16> = get_env_port();
    // Relay no (valid) port was given and that default port is being used
    if port.is_none() {
        println!("No (valid) port given. Using port {DEFAULT_PORT} by default!");
    }
    // Unwrap the option to get user defined port or use the default
    let port: u16 = port.unwrap_or(DEFAULT_PORT);

    (ip_addr, port)
}

pub fn print_info(ip_addr: &[u8; 4], port: &u16) {
    // Instructing users on how to change the variables
    println!("If you wish to change the ip address or port, \
            find the or create a .env file at .exe's root and \
            add (or change) the fields: IP=192.168.X.X and PORT=X."
    );
        
    // Print the ip and port the server is hosted on.
    println!("Server hosted statically on {}:{} ", ip_addr.iter().format("."), port);
    
    // Converts the ip address to a string or "localhost" (if it is so)
    let access_url = if ip_addr == &LOCAL_HOST {
        String::from("localhost")
    } else {
        ip_addr.iter()
        .format(".")
        .to_string() 
    };
    
    // If the port is not port 80 or 443
    let access_port = if port != &80 && port != &443 {
        format!(":{}", port) // Set it to a colon followed by the port
    } else {
        String::from("") // Else set it to none, as those ports are unecessary to browsers
    };

    // Give the website url dictated by the strings above
    println!("Access the website by going to \"{}{}\"", access_url, access_port);
}