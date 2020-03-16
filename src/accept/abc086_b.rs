use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let a: String = read();
    let b: String = read();
    let ab: f64 = (a + &b).parse().ok().unwrap();
    println!("{}",
        if ab.sqrt() == ab.sqrt().round() { "Yes" } else { "No" });
}
