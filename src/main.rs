
use std::fs::File;
use std::io::ErrorKind;

fn file_main() {
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

fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    let third: &i32 = &vector[2];

    println!("Print third data in the  Vec is {:?}", third);
    let third: Option<&i32> = vector.get(2);
    match third {
        Some(third) => println!("The third elemet in {third}"),
        None => println!("The third elemet is NULL"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);

    println!("The first element is: {first}");

    let mut vi = vec![100, 200, 300, 400, 500, 600, 700, 800];
    println!("{:#?}", vi);
    for i in &mut vi {
        *i += 50;
        println!("{:#?}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("Color")),
        SpreadsheetCell::Float(22.21),
    ];
    for x in row {
        println!("{:#?}", x);
    }
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Man_Blue"), 3);
    scores.insert(String::from("Man_Red"), 4);
    let complicated = scores.get("Man_Blue").copied().unwrap_or(2);
    let name_chel: String = String::from("chelsea");
    let scr_chel: u8 = 4;
    scores.insert(name_chel, scr_chel);
    println!("value of man_blue is {:?}", complicated);

    println!("{scores:?}");
    for (name, scr) in &scores {
        println!("{} = {}", name, scr);
    }
    let text = "Hello world and wonderfull world  (❁´◡`❁)`)";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }
    println!("{map:#?}");
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
 use std::fs::File;                                                                 1  use std::io::ErrorKind;                                                            3                                                                                     1  fn main() {                                                                        2      let greeting_file_result = File::open("hello.txt");                            3      let greeting_file = match greeting_file_result {                               4          Ok(file) => file,                                                          5          Err(s_err) => match s_err.kind() {                                         6              ErrorKind::NotFound => match File::create("hello.txt") {               7                  Ok(fc) => fc,                                                      8                  Err(e) => panic!("Problem creaitng file {e:#?} form panic macro    9              },                                                                    10              other_error => {                                                      11                  panic!("Problem creaitng file {other_error:#?} form panic macro   12              }                                                                     13          },                                                                        14      };                                                                            15      println!("finally {greeting_file:#?}")                                        16  }                                                                                  ~                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
