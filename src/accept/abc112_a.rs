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
    let n: u32 = sc.read();
    if n == 1 {
        println!("Hello World");
    } else {
        let a: u32 = sc.read();
        let b: u32 = sc.read();
        println!("{}", a+b);
    }
    
}
