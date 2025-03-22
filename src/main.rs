use std::fs::File;
use std::io::ErrorKind;
fn another_way() {
    let greeting_file = File::open("hellows.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hel|lows.txt").unwrap_or_else(|error| {
                panic!("Problem when creating file: {error:?}");
            })
        } else {
            panic!("Problem opning a file {error:?}");
        }
    });
    println!("finally {greeting_file:#?} from another_way() .");
}
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
    println!("finally {greeting_file:#?}");
    another_way();
}
