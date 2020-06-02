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

fn factorization(n: u64) -> Vec<(u64,u64)> {
    let mut n = n;
    let mut v = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        let mut x = 0;
        while n % p == 0 {
            n /= p;
            x += 1;
        }
        if x != 0 { v.push((p,x)) }
        p += 1;
    }
    if n != 1 { v.push((n,1)) }
    v
}

fn main() {
    input! {
        n: u64
    }
    let fs = factorization(n).into_iter().collect::<Vec<_>>();
    // println!("{:?}", fs);
    let mut res = 0;
    for (_p,e) in fs {
        let mut e = e;
        let mut i = 1;
        while i <= e {
            res += 1;
            e -= i;
            i += 1;
        }
    }
    println!("{}", res);
}
/* vim:set foldmethod=marker: */
