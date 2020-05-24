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
        a: [usize;n]
    }
    let mut v = vec![0;n+1];
    for i in &a {
        v[i-1] += 1;
    }
    let vc = v.iter().map(|a| {
        if *a < 2 { 0 } else { a * (a-1usize) / 2 }
    }).collect::<Vec<usize>>();
    // println!("{:?}", vc);
    let sm = vc.iter().sum::<usize>();
    for i in &a {
        let i = i-1;
        let a = (v[i] as i64) - 1;
        let c = if a < 2 { 0 } else { a * (a-1) / 2 } as usize;
        println!("{}", sm - (vc[i] - c));
    }
}
/* vim:set foldmethod=marker: */
