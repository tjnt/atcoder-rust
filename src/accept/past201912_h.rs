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
    let mut c = Vec::new();
    for _ in 0..n {
        c.push(sc.read::<u64>());
    }
    let q: usize = sc.read();
    let mut v1 = vec![0u64;n];
    let mut v2 = 0u64;
    let mut v3 = 0u64;
    let mut min_all: u64 = *c.iter().min().unwrap();
    let mut min_even: u64 =
        *c.iter().enumerate().filter(|&t| t.0 % 2 == 0)
          .map(|t| t.1).min().unwrap();
    for _ in 0..q {
        match sc.read::<usize>() {
             1 => {
                 let x = sc.read::<usize>() - 1;
                 let a = sc.read::<u64>();
                 let b = if x % 2 == 0 {
                     v1[x] + v2 + v3
                 } else {
                     v1[x] + v3
                 } + a;
                 if b <= c[x] {
                     v1[x] += a;
                     min_all = std::cmp::min(c[x]-b, min_all);
                     if x % 2 == 0 {
                         min_even = std::cmp::min(c[x]-b, min_even);
                     }
                 }
             },
             2 => {
                 let a = sc.read::<u64>();
                 if a <= min_even {
                     v2 += a;
                     min_even -= a;
                     min_all = std::cmp::min(min_all, min_even);
                 }
             },
             3 => {
                 let a = sc.read::<u64>();
                 if a <= min_all {
                     v3 += a;
                     min_all -= a;
                     min_even -= a;
                 }
             },
             _  => panic!()
        }
    }
    let res = v1.iter().sum::<u64>()
            + v2 * (((n as u64) + 1) / 2)
            + v3 * (n as u64);
    println!("{}", res);
}
/* vim:set foldmethod=marker: */
