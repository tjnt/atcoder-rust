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
    let n:i32 = read();
    let mut a: Vec<i32> = (0..n).map(|_| read()).collect();
    a.sort();
    a.reverse();
    let a1 = a.iter().zip(0..n).fold(0, |a, (x,i)| {
        if i % 2 == 0 { a + x } else { a } });
    let a2 = a.iter().zip(0..n).fold(0, |a, (x,i)| {
        if i % 2 != 0 { a + x } else { a } });
    println!("{}", (a1 - a2).abs());
}
