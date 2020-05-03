use std::io::*;
use std::str::FromStr;

struct Scanner<R: Read> {
    reader: R,
}

impl<R: Read> Scanner<R> {
    fn new(r: R) -> Scanner<R> {
        Scanner { reader: r }
    }
    fn read<T: FromStr>(&mut self) -> T {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut vv = vec![0;n];
    for _ in 0..k {
        let d: usize = sc.read();
        for _ in 0..d {
            let a: usize = sc.read();
            vv[a-1] += 1;
        }
    }
    let res = vv.iter().filter(|v| {**v == 0 }).count();
    println!("{}", res);
}
