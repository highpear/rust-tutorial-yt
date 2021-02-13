extern crate rand;
use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 11);  // 1 to 10

    println!("random number: {}", rand_num);

    // Flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("random boolean: {}", random_bool);

}
