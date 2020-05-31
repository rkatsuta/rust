fn main() {
    let s = "Hello".to_string();
    // compile error
    //myprint(s);
    //myprint(s);
    // access by refenece
    myprint(&s);
    myprint(&s);
}

// complile error because of move semantics
//fn myprint<T: std::fmt::Display>(msg: T) {

// msg accepted by reference.
fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg);
}
