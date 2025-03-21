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
