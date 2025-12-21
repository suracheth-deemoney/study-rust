use std::fs;
use std::io;

fn main() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
