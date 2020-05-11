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

use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize1,
        aa: [usize1;n]
    }
    let mut m = HashMap::new();
    let mut v = Vec::new();
    let mut i = 0;
    loop {
        let t = (i, aa[i]);
        if let Some(i) = m.get(&t) {
            if *i == 2 { break; }
        }
        v.push(i);
        *m.entry(t).or_insert(0) += 1;
        i = t.1;
    }
    let k = k+1;
    if k < v.len() {
        println!("{}", v[k] + 1);
    } else {
        let l = m.iter().filter(|t| *t.1 == 1).count();
        let r = m.iter().filter(|t| *t.1 == 2).count();
        let v = &v[l..(v.len()-r)];
        println!("{}", v[(k-l) % v.len()] + 1);
    }
}
/* vim:set foldmethod=marker: */
