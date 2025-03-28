fn main() {
    let string1: String = String::from("Long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}")
}

fn longest<'a>(str_x: &'a str, str_y: &'a str) -> &'a str {
    if str_x.len() > str_y.len() {
        str_x
    } else {
        str_y
    }
}
