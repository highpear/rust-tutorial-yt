extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

// use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool
}

fn main() {
    let json_str = r#"
        {
            "name": "Taro",
            "age": 21,
            "is_male": true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("The name is {}", p.name);
        println!("The age is {}", p.age);
        println!("Is the person male?: {}", p.is_male);        
    } else {
        println!("cannot parse the JSON");
    }
}
