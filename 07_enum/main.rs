enum Directions {

    Up,
    Down,
    Left,
    Right,
    
}

fn main() {

    let player_direction:Directions = Directions::Up;
    
    match player_direction {
        Directions::Up => println!("We are heading up!"),
        Directions::Down => println!("We are going all the way down.."),
        Directions::Left => println!("Left is right"),
        Directions::Right => println!("Moving towards the right"),
    }
}