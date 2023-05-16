use std::fs::{File, self};
use std::io::{ErrorKind, Read, self};

fn main() {
    let greet_file = File::open("hello.txt");

    let mut greet_file = match greet_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created) => created,
                Err(error) => panic!("creation error {}", error)
            }
            other_err => panic!("Problem with oppening file: {other_err}")
        }
    };
    let mut username = String::new();
    match greet_file.read_to_string(&mut username) {
        Ok(bytes) => println!("Read {bytes} bytes"),
        Err(e) => panic!("Error {e}"),
    };
    println!("Read username: {username}");
    match read_from_file("happy") {
        Ok(text) => println!("We read it: \n {text}"),
        Err(err) => panic!("Your read crashed: {err}"),
    }
}

fn read_from_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
