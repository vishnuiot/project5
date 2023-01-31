enum Direction {
    up,
    down,
    right,
    left,
}

fn main() {
    let player_direction: Direction = Direction::up;
    match player_direction {
        Direction::up => println!("This is direction up"),
        Direction::down => println!("This is direction down"),
        Direction::right => println!("Thisi is direction right"),
        Direction::left => println!("This is direaction left"),
    }
    println!("test program");
}