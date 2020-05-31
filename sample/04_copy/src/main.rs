fn main() {
    // copy semantics beacause of Scaler type
    // Scaler Type has the copy trait
    //let x = 1;
    //let y = x; // これはOK
    //println!("x = {}", x);
    //println!("y = {}", y);
    //
    // move semantics
    let x = "hello world".to_string();
    let y = x;
    println!("{}", x);
    println!("{}", y);
}
