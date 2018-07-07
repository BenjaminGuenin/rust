#[derive(Debug)]
struct Color {
    red: u8, // u8: 0 to 255
    green: u8,
    blue: u8,
}

fn main() {
    let bg = Color {red: 255, green: 70, blue: 15};
    print!("color is {}", bg.red);
    bg.blue = 45;
}
