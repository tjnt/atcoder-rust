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

pub fn bit_search(v: &[i32], c: &[i32]) -> i32 {
    let n = v.len();
    let mut res = 0;
    let mut b: i32 = 1;
    while b < (1i32 << n) {
        let mut val = 0;
        let mut cst = 0;
        for i in 0..n {
            if b & (1 << i) != 0 {
                val += v[i];
                cst += c[i];
            }
        }
        res = std::cmp::max(res, val - cst);
        b += 1;
    }
    res
}

fn main() {
    input! {
        n: usize,
        v: [i32;n],
        c: [i32;n],
    }
    println!("{}", bit_search(&v, &c));
}
