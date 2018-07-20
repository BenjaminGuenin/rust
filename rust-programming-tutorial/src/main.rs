fn main() {

    let immutable_string = "hello"; // cannot be modified - stored on the stack
    let mut mutable_string = String::from("hello"); // can be modified - stored on the heap

    mutable_string.push_str(" world!");
    println!("{}", mutable_string);

    // Scope ownership memory management
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid. The memory taken by s is fred.

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // Error, Rust prevents us to use invalidated reference
    println!("{}, world!", s2);

}