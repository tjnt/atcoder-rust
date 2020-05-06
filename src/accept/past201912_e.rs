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
    let q: usize = sc.read();

    let mut g = vec![vec![false;n];n];
    for _ in 0..q {
        match sc.read() {
            1 => {
                let a = sc.read::<usize>() - 1;
                let b = sc.read::<usize>() - 1;
                g[a][b] = true;
            },
            2 => {
                let a = sc.read::<usize>() - 1;
                for i in 0..n {
                    if g[i][a] {
                        g[a][i] = true;
                    }
                }
            },
            3 => {
                let a = sc.read::<usize>() - 1;
                let is: Vec<usize> =
                    (0..n).filter(|&i| g[a][i]).collect();
                for i in is {
                    for j in 0..n {
                        if g[i][j] {
                            if a == j { continue; }
                            g[a][j] = true;
                        }
                    }
                }
            }
            _ => panic!()
        };
    }
    for l in g {
        for c in l {
            print!("{}", if c {"Y"} else {"N"});
        }
        println!();
    }
    
}
/* vim:set foldmethod=marker: */
