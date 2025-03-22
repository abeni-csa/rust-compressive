
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(s_err) => match s_err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creaitng file {e:#?} form panic macro"),
            },
            other_error => {
                panic!("Problem creaitng file {other_error:#?} form panic macro");
            }
        },
    };
    println!("finally {greeting_file:#?}")
}

