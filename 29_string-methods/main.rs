fn main() {
    // replace
    {
        let my_str = String::from("Rust is fantastic!");
        println!("After replace: {}", my_str.replace("fantastic", "great"));
    }

    // lines
    {
        let my_str = String::from("Tha weahter is\nnice\noutside mate!");

        for line in my_str.lines() {
            println!("[ {} ]", line);
        }
    }

    // split
    {
        let my_str = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_str.split("+").collect();
        println!("{}", my_str);
        println!("at index 2: {}", tokens[2]);
    }

    // trim
    {
        let my_str = String::from("   My name is Taro   \n\r");
        println!("Befor trim: {}", my_str);
        println!("After trim: {}", my_str.trim());
    }

    // chars
    {
        let my_str = String::from("Rust!");
        println!("str: {}", my_str);

        match my_str.chars().nth(4) {
            Some(c) => println!("char at index 4: {}", c),
            None => println!("no char at index 4")
        }

    }

    
}