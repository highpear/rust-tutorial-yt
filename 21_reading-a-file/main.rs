use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut file = File::open("info.txt").expect("cannot open file !");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file !");

    println!("File contents\n\n{}", contents);

}