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
    let n:u32 = read();
    let a: Vec<u32> = (0..n).map(|_| read()).collect();
    let mut num = 999;
    for e in a {
        let mut nm = 0;
        let mut v = e;
        while v % 2 == 0 {
            v /= 2;
            nm += 1;
        }
        num = std::cmp::min(num,nm)
    }
    println!("{}", num);
}
