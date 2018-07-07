fn main() {
    let mut x = 10;
    let xr = &x;
    let dom = &x;
    
    dom +=1; // Will not work because the reference is not mutable.

    println!("x is {}", x);
    println!("x is {}", xr);
    println!("x is {}", dom);

}
