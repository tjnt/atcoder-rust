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
    let n: i32 = read();
    let y: i32 = read();
    for i in 0..n+1 {
        for j in 0..n-i+1 {
            let k = n-i-j;
            if k < 0 {
                continue;
            }
            if i*10000 + j*5000 + k*1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    print!("-1 -1 -1");
}
