/* {{{ */
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
/* }}} */

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let s = sc.read::<String>().chars().collect::<Vec<char>>();
    let q: usize = sc.read();
    let mut vl = Vec::new();
    let mut vr = Vec::new();
    let mut flg = true;
    for _ in 0..q {
        let t: usize = sc.read();
        if t == 1 {
            flg = !flg;
        } else {
            let f: usize = sc.read();
            let s: char = sc.read::<String>().chars().next().unwrap();
            if f == 1 {
                if flg { vl.push(s); }
                else   { vr.push(s); }
            } else {
                if flg { vr.push(s); }
                else   { vl.push(s); }
            }
        }
    }
    vl.reverse();
    let it = vl.iter().chain(s.iter()).chain(vr.iter());
    if flg {
        print!("{}", it.cloned().collect::<String>());
    } else {
        print!("{}", it.rev().cloned().collect::<String>());
    }
}
/* vim:set foldmethod=marker: */
