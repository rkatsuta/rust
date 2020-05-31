fn main() {
    for i in 1..=30 {
        println!("Hello, {} world!", to_ordinal_string(i));
    }
}

fn to_ordinal_string(n: usize) -> String {
    let s = match n % 10 {
        // matchの左辺で結果を代入できる
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{:04}{}", n, s) // ;がないことでreturnと同義
}
