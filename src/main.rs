use std::io::{stdin, stdout, Write};

mod list_manipulation;

fn main() {
    let mut s = String::new();
    print!("Let's make a todo list for you. Please enter in an item: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    println!("You typed: {}",s);
    let new_file = list_manipulation::create_file();
    
}