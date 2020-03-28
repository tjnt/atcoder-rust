macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

pub fn ncr(n: u64, r: u64) -> u64 {
    (n - r + 1..n + 1).product::<u64>() / (1..r + 1).product::<u64>()
}

use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ss: [chars;n]
    }
    let march: Vec<char> = "MARCH".chars().collect();
    let v: Vec<char> =
        ss.iter().map(|s| s[0]).filter(|c| march.iter()
          .any(|m| *m == *c)).collect();
    let mut m: HashMap<char,u64> = HashMap::new();
    for x in v {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut cnt = 0;
    let v: Vec<u64> = m.iter().map(|t| *t.1).collect();
    for (i,a) in v.iter().enumerate() {
        for (j,b) in v.iter().enumerate() {
            for (k,c) in v.iter().enumerate() {
                if i != j && j != k && k != i{
                    cnt += a * b * c;
                }
            }
        }
    }
    println!("{}", cnt / 6);
}
