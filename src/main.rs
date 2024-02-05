// use std::io::{stdin, stdout, Write};
mod list_manipulation;
//use list_manipulation::TodoList;

// fn main() {
//     let mut s = String::new();
//     print!("Let's make a todo list for you. Please enter in an item: ");
//     let _ = stdout().flush();
//     stdin().read_line(&mut s).expect("Did not enter a correct string");

//     println!("You typed: {}",s);
//     let new_file = list_manipulation::create_file();
    
// }

use std::env;

use crate::list_manipulation::TodoList;

fn main() {
    let mut todo_list = TodoList::new();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() >= 1 { 
        let command: &String = &args[1];

        match &command[..] {
            "create" => todo_list.create_file(),
            "add" => todo_list.add_item(&args[2..]),
            //"remove" => todo_list.remove_items(&args[2..].to_vec()),
            "list" => todo_list.list_items(),
            "reset" => todo_list.create_file(),
            "--help" => TodoList::help(),
            &_ => todo_list.read_from_file()
        } 
    } else {
        TodoList::help();
    }
}

// fn test() {
//     // Create a mutable string to store user input
//     let mut input = String::new();

//     // Print a prompt without a new line
//     print!("Please enter some text: ");

//     // Flush the output to ensure the prompt is displayed
//     io::stdout().flush().unwrap();

//     // Read user input into the string
//     io::stdin().read_line(&mut input)
//         .expect("Failed to read line");

//     // Print the user input
//     println!("You entered: {}", input);
// }