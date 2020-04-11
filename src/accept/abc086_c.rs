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
    let v: Vec<(i32,i32,i32)> = (0..n).map(|_| {
        (read(),read(),read())
    }).collect();

    let mut p = (0,0,0);
    for (t,x,y) in v {
        let tp = t-p.0;
        let xp = (x-p.1).abs();
        let yp = (y-p.2).abs();
        if xp + yp <= (t-p.0) && (tp + xp + yp) % 2 ==0  {
            p = (t,x,y);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
