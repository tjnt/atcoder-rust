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
    let m: usize = sc.read();
    let mut sss = Vec::new();
    for _ in 0..m {
        let k: usize = sc.read();
        let mut ss = Vec::new();
        for _ in 0..k {
            ss.push(sc.read::<usize>());
        }
        sss.push(ss);
    }
    let mut pp = Vec::new();
    for _ in 0..m {
        pp.push(sc.read::<usize>());
    }

    let mut res = 0;
    let mut b: usize = 0;
    while b < (1usize << n) {
        let mut num = 0;
        for (i, ss) in sss.iter().enumerate() {
            let mut cnt = 0;
            for s in ss {
                if b & (1usize << (s-1)) != 0 {
                    cnt += 1;
                }
            }
            if cnt % 2 == pp[i] {
                num += 1;
            }
        }
        if num == m {
            res += 1;
        }
        b += 1;
    }
    println!("{}", res);
}
