fn main() {

    let num = 20;

    match num {
        1 => println!("It is one!"),
        2..=20 => println!("It is greater than one!"),
        // 10 | 11 => println!("It is either 10 or 11"),
        _ => println!("It doesn't match!")
    }
}