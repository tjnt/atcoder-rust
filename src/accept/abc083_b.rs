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

fn keta_sum(mut v:i32) -> i32 {
    let mut n:i32 = 0;
    while v != 0 {
        n += v % 10;
        v /= 10;
    }
    n
}

fn main() {
    let n:i32 = read();
    let a:i32 = read();
    let b:i32 = read();
    let sum:i32 = (1..n+1).filter(|i| {
        let k:i32 = keta_sum(*i);
        a <= k && k <= b
    }).sum();
    println!("{}", sum);
}
