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

use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        aa: [u64;n]
    }
    let mut m = BTreeMap::new();
    for a in &aa {
        *m.entry(a).or_insert(0) += 1u64;
    }
    let v: Vec<(&u64,u64)> = m.into_iter().filter(|&(_k,v)| v >= 2u64)
                                   .rev().take(2).collect();
    if v.len() < 2 {
        println!("0");
    } else if v[0].1 >= 4 {
        println!("{}", v[0].0 * v[0].0);
    } else {
        println!("{}", v[0].0 * v[1].0);
    }
}
