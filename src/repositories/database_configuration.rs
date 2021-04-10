extern crate dotenv;
use dotenv::dotenv;
use mongodb::{bson::doc,sync::{Client,Database}};
use std::env;
use std::error::Error;

pub fn init() ->Result<Database,Box<dyn Error>>{
    dotenv().ok();
    let mongodb_type = env::var("MONGODB_TYPE").expect("MONGODB_TYPE missing");
    let mongodb_address = env::var("MONGODB_ADDRESS").expect("MONGODB_ADDRESS missing");
    let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
    let mongodb_user = env::var("MONGODB_USER").expect("MONGODB_USER missing");
    let mongodb_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD missing");
    let mongodb_options = env::var("MONGODB_OPTIONS").expect("MONGODB_OPTIONS missing");
    let uri=String::new();
    let uri = uri+&mongodb_type+"://"+&mongodb_user+":"+&mongodb_password+"@"+&mongodb_address+"/"+&database+&mongodb_options;
    let client = Client::with_uri_str(
        &uri
    )?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Connected to MongoDB successfully.");
    Ok(client.database(&database))
}