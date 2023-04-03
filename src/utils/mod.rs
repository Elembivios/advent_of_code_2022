pub mod tree;
pub mod point;

use std::io;

pub fn wait_user_input() {
    println!("(Press ENTER to continue..)");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).ok().unwrap(); 
}