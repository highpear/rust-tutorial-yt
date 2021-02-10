fn main() {

    let mut n = 0;
    
    while n < 50 {
        n += 1;

        if n % 5 == 0 {
            println!("n is {}", n);
        }
    }
}