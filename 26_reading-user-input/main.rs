use std::io;

fn main() {
    let mut input = String::new();

    println!("please input >>");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! input:{}", input);
        },
        Err(_e) => println!("Something went wrong!")

    }


}