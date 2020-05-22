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

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u64;n]
    }
    let mut s = p.iter().scan(0u64, |a,p| {
        *a += *p;
        Some(*a)
    }).collect::<Vec<u64>>();
    s.insert(0,0);

    let mut mx: u64 = 0;
    let mut mi: usize = 0;
    for i in k..n+1 {
        let v = s[i] - s[i-k];
        if mx < v {
            mx = v;
            mi = i;
        }
    }
    mi = mi - k;

    let t = (1..1000+1).scan(0.5f64, |a, _| {
        *a += 0.5;
        Some(*a)
    }).collect::<Vec<f64>>();
    let mut res: f64 = 0.0;
    for i in mi..mi+k {
        res += t[p[i] as usize - 1];
    }
    println!("{}", res);
}
/* vim:set foldmethod=marker: */
