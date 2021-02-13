use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // Add values
    marks.insert("Math", 96);
    marks.insert("English", 60);
    marks.insert("Science", 80);
    marks.insert("Social Studies", 70);

    // find length of HashMap
    println!("length: {}", marks.len());

    // get a single value
    match marks.get("Science") {
        Some(mark) => println!("You got {}", mark),
        None => println!("Unknown...")
    }

    // remove a value
    marks.remove("Social Studies");

    for (key, value) in &marks {
        println!("key {}, value {}", key, value);
    }

    // check for value
    println!("You scored English ? {}", marks.contains_key("English"));
    
}