#[derive(Debug)]
struct Color {
    red: u8, // u8: 0 to 255
    green: u8,
    blue: u8,
}

fn main() {
    let mut bg = Color {red: 255, green: 70, blue: 15};
    print!("color is {}", bg.blue);
    bg.blue = 45; // impossible, because we need an mutable struct.
    print!("color is {}", bg.blue);
}
