/* {{{ */
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
/* }}} */

fn bisection_method<F>(l: u64, r: u64, pred: F) -> u64
    where F : Fn(u64) -> bool {
    let mut l = l;
    let mut r = r;
    while r - l != 1 {
        let m = l + (r-l) / 2;
        if pred(m) { l = m } else { r = m }
    }
    l
}

pub fn digit_num(mut n: u64) -> u64 {
    let mut k = 0;
    let d = 10;
    while n != 0 {
        k += 1;
        n /= d;
    }
    k
}

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    println!("{}",
        bisection_method(0, 10u64.pow(9)+1, |n| {
            a * n + b * digit_num(n) <= x
        }));
}
/* vim:set foldmethod=marker: */
