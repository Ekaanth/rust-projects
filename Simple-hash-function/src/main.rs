use std::env;
use sha256::digest;

fn main() {
    let input = "hello";
    let val = digest(input);
    println!("{}",val);

    let input = "hello".to_string();
    let val = digest(input);
    println!("{}",val);


    let input = b"hello";
    let val = digest(input);
    println!("{}",val);

}
