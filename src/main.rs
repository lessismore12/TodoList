mod list_manipulation;


use std::env;

use crate::list_manipulation::{does_file_exist, TodoList};

fn main() {
    let mut todo_list = TodoList::new();
    
    if !does_file_exist() {
        todo_list.create_file();
    }

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 1 { 
        let command: &String = &args[1];

        match &command[..] {
            "create" => todo_list.create_file(),
            "add" => todo_list.add_item(&args[2..]),
            "remove" => todo_list.remove_items(args[2..].to_vec()),
            "list" => todo_list.list_items(),
            "reset" => todo_list.create_file(),
            "--help" => TodoList::help(),
            &_ => todo_list.read_from_file()
        } 
    } else {
        TodoList::help();
    }
}
