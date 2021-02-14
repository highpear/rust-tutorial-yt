extern crate regex;

use regex::Regex;

fn main() {

    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "ABCDE";

    match re.captures(text) {
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),  // = &caps[0]
        None => println!("Not matched")
    }
}
