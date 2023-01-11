use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    println!("Search for {}", query);
    println!("in file {}", filename);
    let contents = fs::read_to_string(filename)
                            .expect("Some went wrong reading the file");
    println!("with text: \n {}", contents);
}
