fn main() {

    let animals = vec!["Cat", "Dog", "Rabbit"];

    for (i, animal) in animals.iter().enumerate() {

        println!("{} the animal is {}", i + 1, animal);
    }
}