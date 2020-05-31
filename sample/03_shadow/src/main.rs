fn main() {
    let x = 1; // place memory with 1

    {
        let x = x + 1;
        println!("inside x {}", x);
    }
    println!("outside x = {}", x);

    let x = x + 2; // replace new memory place with 3. named x. another line2.
    println!("outside x2 = {}", x);
}
