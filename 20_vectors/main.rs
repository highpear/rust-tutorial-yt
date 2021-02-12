fn main() {
    let mut my_vec = vec![1, 2, 3, 4];

    for val in my_vec.iter() {
        println!("{}", val);
    }

    my_vec.push(100);
    my_vec.remove(0);

    for val in my_vec.iter() {
        println!("{}", val);
    }
    
}