fn main() {

    let mut x = 10;

    {
        let x = 100;
        println!("x is {}", x);
    }

    let x = "X is a string";
    println!("{}", x);

    let x = true;
    println!("x is {}", x);
}