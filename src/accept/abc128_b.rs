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

    let mut sps = Vec::new();
    for i in 1..n+1 {
        let s: String = sc.read();
        let p: u32 = sc.read();
        sps.push((i, s, p));
    }
    sps.sort_by(|l,r| {
        if l.1 == r.1 {
            r.2.partial_cmp(&l.2).unwrap()
        } else {
            l.1.partial_cmp(&r.1).unwrap()
        }
    });
    for (i, _, _) in sps {
        println!("{}", i);
    }
}
