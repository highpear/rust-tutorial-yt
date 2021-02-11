fn main() {

    let mut x = 10;
    let x_ref = &mut x;

    {
        *x_ref += 1;
    }

    println!("x is {}", x);
}