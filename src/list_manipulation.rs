use std::{fs::{File, OpenOptions}, path::Path, io::{Read, Write, BufWriter}};


#[allow(unused)]
const FILE_PATH: &str = "todo.txt";

pub struct TodoList {
    file: File,
    file_path: String,
}

impl TodoList {

    pub fn new() -> Self {
        if does_file_exist() {
            Self {
                file: File::open(FILE_PATH)
                .expect(&format!("file not found: {}", FILE_PATH)),
                file_path: FILE_PATH.to_string()
            }
        } else {
            println!("I see you don't have a list started. I'll make one for you");
            Self {
                file: File::create(FILE_PATH).expect("Failed to create file"),
                file_path: FILE_PATH.to_string()
            }
        }
    }

    pub fn add_item(&mut self, items: &[String]) { 
        let file = OpenOptions::new().append(true).open(self.file_path.clone()).unwrap();
        let mut buffer = BufWriter::new(file);
        for item in items {
            let line = format!("{}\n", item);
            let _ = buffer.write_all(line.as_bytes());
        }
       
    }

    pub fn create_file(&mut self) {
        self.file = File::create(FILE_PATH).expect("Failed to create file");
    }

    pub fn read_from_file(&self) {
        let mut file = File::open(FILE_PATH)
            .expect(&format!("file not found: {}", FILE_PATH));
    
        let mut contents = String::new();
    
        file.read_to_string(&mut contents)
            .expect(&format!("cannot read file {}", FILE_PATH));
    
        println!("{}", contents);
    }

    #[allow(unused)]
    fn get_contents(&mut self) {
    
        let mut contents = String::new();
    
        self.file.read_to_string(&mut contents)
            .expect(&format!("cannot read file {}", FILE_PATH));
    
        for line in contents.lines() {
            print!("{:?}", line)
        }
    }

    const TODO_HELP: &'static str = "Usage: todo [COMMAND] [ARGUMENTS]
        Todo is a super fast and simple tasks organizer written in rust
        Example: todo list
        Available commands:
            - add [TASK/s]
                adds new task/s
                Example: todo add \"buy carrots\"
            - list
                lists all tasks
                Example: todo list
            - done [INDEX]
                marks task as done
                Example: todo done 2 3 (marks second and third tasks as completed)
            - rm [INDEX]
                removes a task
                Example: todo rm 4
            - reset
                deletes all tasks
            - restore 
                restore recent backup after reset
            - sort
                sorts completed and uncompleted tasks
                Example: todo sort
            - raw [todo/done]
                prints nothing but done/incompleted tasks in plain text, useful for scripting
                Example: todo raw done
        ";
    pub fn help() {
        // For readability
        println!("{}", Self::TODO_HELP);
    }
    
}


#[allow(unused)]
fn does_file_exist() -> bool {
    Path::new(FILE_PATH).exists()
}