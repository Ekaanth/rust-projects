use std::io;
use std::io::prelude::*;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let mut x = 0;
    let mut y = 0;
    let name = &name.to_lowercase();
    let chars: Vec<char> = name.chars().collect();
    for (index, element) in chars.iter().enumerate() {
        if *element == 'z' {
            x += 1;
        }else if *element == 'o'{
            y += 1;
        }
    }
    if x*2 == y {
        println!("Yes");
    }else{
        println!("No");
    }
}
