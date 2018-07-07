fn main() {

    let mut x = 10;

    let dom = &mut x;

    *dom += 1;

    println!("x is {}", dom);


}
