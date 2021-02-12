struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {}. I am {} years old.", self.name, self.age);
    }
}


fn main() {

    let taro = Person { name: String::from("Taro"), age: 23};
    
    println!("{}", taro.to_string());   
    
}