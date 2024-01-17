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

    println!("{:?}", &args[1]);
    if args.len() >= 1 { 
        let command: &String = &args[1];

        match &command[..] {
            "create" => todo_list.create_file(),
            "add" => todo_list.add_item(&args[2..]),
            "remove" => todo_list.remove_items(&args[2..]),
            "--help" => TodoList::help(),
            &_ => todo_list.read_from_file()
        } 
    } else {
        TodoList::help();
    }
}