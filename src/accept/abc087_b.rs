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
    let a:i32 = read();
    let b:i32 = read();
    let c:i32 = read();
    let x:i32 = read();
    let mut cnt = 0;
    for i in 0..a+1 {
        let x = x - 500 * i;
        if x < 0 { continue; }
        for j in 0..b+1 {
            let x = x - 100 * j;
            if x < 0 { continue; }
            if x % 50 == 0 && x / 50 <= c {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
