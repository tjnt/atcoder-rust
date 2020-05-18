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
    let n: usize = sc.read();
    let mut v = vec![vec![None;n];n];
    for i in 0..n {
        let a: usize = sc.read();
        for _ in 0..a {
            let x = sc.read::<usize>() - 1;
            let y = if sc.read::<usize>() == 0 {false} else {true};
            v[i][x] = Some(y);
        }
    }
    let mut res = 0;
    for b in 0..(1u64 << n) {
        let mut h = vec![false;n];
        for i in 0..n {
            if b & (1 << i) != 0 {
                h[i] = true;
            }
        }
        let mut ok = true;
        for i in 0..n {
            if !h[i] { continue; }
            for j in 0..n {
                if let Some(y) = v[i][j] {
                    if h[j] != y {
                        ok = false;
                        break;
                    }
                }
            }
        }
        if ok {
            let cnt = h.iter().filter(|b| **b).count();
            res = std::cmp::max(res, cnt);
        }
    }
    println!("{}", res);
}
/* vim:set foldmethod=marker: */
