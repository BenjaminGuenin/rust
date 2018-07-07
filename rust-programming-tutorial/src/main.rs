fn main() {

    let mut x = 10;

    let dom = &mut x;
    *dom += 1;
    println!("x is {}", x);

    // We can't define a mutable reference to x and borrow it at the same time.

}
