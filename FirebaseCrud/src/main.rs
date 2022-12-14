use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u32,
    email:String
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    name: String
}


#[tokio::main]
async fn main() {
    let user = User {
        name: "Abhishek".to_string(),
        age: 30,
        email: "test.email@gmail.com".to_string(),
    };




}
