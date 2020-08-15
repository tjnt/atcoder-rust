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

const N: usize = 4;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut v = vec![vec![' ';N];N];
    for i in 0..N {
        for j in 0..N {
            let c = sc.read::<char>();
            v[N-i-1][N-j-1] = c;
        }
    }
    for l in v {
        for c in l {
            print!("{} ", c);
        }
        println!();
    }
}
/* vim:set foldmethod=marker: */
