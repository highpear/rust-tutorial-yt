use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("cannot create file !");

    file.write_all(b"This is a test string !").expect("cannot write to the file !");
}