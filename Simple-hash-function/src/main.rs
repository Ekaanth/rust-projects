use std::env;
use sha256::digest;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{}", args.len());
    if args.len() == 1 {
        println!("Usage: Please enter one text to hash");
        return;
    }
    for i in 0..args.len() {
        let args_1 = &*args[i];
        let mut hasher = digest(args_1.to_string());
        println!("{} => {}",args[i], hasher);
    }
}
