struct Colour {
    red: u8,
    blue: u8,
    green: u8,
}
fn main() {
    let mut bg = Colour { red: 255, blue: 60, green: 80 };
    bg.green=200;
    println!("{},{},{}", bg.red, bg.blue, bg.green);
}