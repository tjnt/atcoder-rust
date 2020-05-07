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
    let mut tbl = vec![vec![0;n];n];
    for i in 0..n {
        for j in i+1..n {
            tbl[i][j] = sc.read::<i64>();
            tbl[j][i] = tbl[i][j];
        }
    }

    let calc = |v: &[usize]| {
        let mut res = 0;
        for i in 0..v.len() {
            for j in i+1..v.len() {
                res += tbl[v[i]][v[j]];
            }
        }
        res
    };

    use std::cmp::max;
    let mut res = std::i64::MIN;
    let mask = (1 << n) - 1;
    for b1 in 0..(1u64 << n) {
        for b2 in 0..(1u64 << n) {
            if b1 & b2 != 0 {
                continue;
            }
            let b3 = mask ^ b1 ^ b2;
            let mut v1 = Vec::new();
            let mut v2 = Vec::new();
            let mut v3 = Vec::new();
            for i in 0..n {
                if b1 & (1 << i) != 0 {
                    v1.push(i);
                }
                if b2 & (1 << i) != 0 {
                    v2.push(i);
                }
                if b3 & (1 << i) != 0 {
                    v3.push(i);
                }
            }
            res = max(calc(&v1) + calc(&v2) + calc(&v3), res);
        }
    }

    println!("{}", res);
}
/* vim:set foldmethod=marker: */
