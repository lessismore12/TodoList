use std::{fs::{File, OpenOptions}, path::Path, io::{Read, Write}};

#[allow(unused)]
const FILE_PATH: &str = "todo.txt";

#[allow(unused)]
fn does_file_exist() -> bool {
    Path::new(FILE_PATH).exists()
}

#[allow(unused)]
pub fn create_file() -> File {
    let file = File::create(FILE_PATH).expect("Failed to create file");
    file
}

#[allow(unused)]
pub  fn add_item(list: String, item: String) -> String {
    [list, item].join("\n")
}

#[allow(unused)]
pub fn read_from_file(file_path: &str) -> String {
    let mut file = File::open(file_path)
        .expect(&format!("file not found: {}", file_path));

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", file_path));

    contents
}

#[allow(unused)]
pub fn write_to_file(file_path: &str) {
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    file.write("buf\n".as_bytes()).expect(&format!("cannot write to file {}", file_path));
}