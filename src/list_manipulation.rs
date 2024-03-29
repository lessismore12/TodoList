use std::{fs::{File, OpenOptions}, io::{BufWriter, Read, Write}, path::Path};

const FILE_PATH: &str = "todo.txt";

pub struct TodoList {
    list: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        if does_file_exist() {
            Self {
                list: get_contents()
            }
        } else {
            println!("I see you don't have a list started. I'll make one for you");
            Self {
                list: Vec::new()
            }
        }
    }

    pub fn add_item(&mut self, items: &[String]) { 
        let mut temp = String::from("");
        let mut new_items: Vec<String> = Vec::new();
        for i in 0..items.len() {
            if items[i].starts_with("--") {
                if temp != "" {
                    new_items.push(temp.to_string().trim().to_owned());
                }
                temp = String::from("");
                temp.push_str(&items[i][2..]);
            } else {
                temp.push_str(&(" ".to_owned() + &items[i]));
            } 

            if i == items.len()-1 {
                new_items.push(temp.to_string().trim().to_owned());
            }
        }
        self.append_list(new_items);        
    }

    fn append_list(&mut self, items_to_add: Vec<String>) {
        let file = OpenOptions::new().append(true).open(FILE_PATH).unwrap();
        let mut buffer = BufWriter::new(file);
        
        for item in items_to_add {
            print!("{}", item);
            let line = format!("{}\n", item);
            let _ = buffer.write(line.as_bytes());
            self.list.push(item);
        }
    }

    pub fn remove_items(&mut self, mut lines_to_remove: Vec<String>) {

        lines_to_remove.sort_by(|a, b| b.cmp(a));
        for line in lines_to_remove {
            let index_to_remove: usize = line.parse::<i32>().unwrap().try_into().unwrap();
            self.list.remove(index_to_remove-1);
        }

        self.create_file();
        let file = OpenOptions::new().write(true).open(FILE_PATH).unwrap();    let mut buffer = BufWriter::new(file);
        
        for item in &self.list {
            let line = format!("{}\n", item);
            let _ = buffer.write(line.as_bytes());
        }
        print!("Current list: {:?}", self.list);
    }

    pub fn create_file(&mut self) {
        File::create(FILE_PATH).expect("Failed to create file");
    }

    pub fn read_from_file(&self) {
        let mut file = File::open(FILE_PATH)
            .expect(&format!("file not found: {}", FILE_PATH));
    
        let mut contents = String::new();
    
        file.read_to_string(&mut contents)
            .expect(&format!("cannot read file {}", FILE_PATH));
    
        println!("{}", contents);
    }


    pub fn list_items(&self) {
        let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();
        file_contents.lines().for_each(|line| println!("{}", line))
    }

    const TODO_HELP: &'static str = "Usage: todo [COMMAND] [ARGUMENTS]
        Todo is a super fast and simple tasks organizer written in rust
        Example: todo list
        Available commands:
            - add [TASK/s]
                adds new task/s
                Example: todo add \"--go to the the supermarket --do laundry\"
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
        ";
    pub fn help() {
        // For readability
        println!("{}", Self::TODO_HELP);
    }
    
}


pub fn does_file_exist() -> bool {
    Path::new(FILE_PATH).exists()
}

fn get_contents()-> Vec<String> {
    
    let mut list_of_contents: Vec<String>= Vec::new();
    let mut contents = String::new();
    let mut file = File::open(FILE_PATH)
            .expect(&format!("file not found: {}", FILE_PATH));

    file.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", FILE_PATH));

    for line in contents.lines() {
        list_of_contents.push(line.to_string())
    }

    list_of_contents
}

// const TODO_HELP: &'static str = "Usage: todo [COMMAND] [ARGUMENTS]
//         Todo is a super fast and simple tasks organizer written in rust
//         Example: todo list
//         Available commands:
//             - add [TASK/s]
//                 adds new task/s
//                 Example: todo add \"--go to the the supermarket --do laundry\"
//             - list
//                 lists all tasks
//                 Example: todo list
//             - done [INDEX]
//                 marks task as done
//                 Example: todo done 2 3 (marks second and third tasks as completed)
//             - rm [INDEX]
//                 removes a task
//                 Example: todo rm 4
//             - reset
//                 deletes all tasks
//             - restore 
//                 restore recent backup after reset
//             - sort
//                 sorts completed and uncompleted tasks
//                 Example: todo sort
//             - raw [todo/done]
//                 prints nothing but done/incompleted tasks in plain text, useful for scripting
//                 Example: todo raw done
//         ";