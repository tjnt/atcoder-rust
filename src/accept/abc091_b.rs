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

use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String;n],
        m: usize,
        t: [String;m]
    }

    let mut a: HashMap<String,i32> = HashMap::new();
    let mut b: HashMap<String,i32> = HashMap::new();
    for k in s {
        let v = a.entry(k).or_insert(0);
        *v += 1;
    }
    for k in t {
        let v = b.entry(k).or_insert(0);
        *v += 1;
    }
    let mut res = 0;
    for (k,v1) in a {
        let v2 = if let Some(v) = b.get(&k) { *v } else { 0 };
        res = std::cmp::max(res, v1-v2);
    }
    println!("{}", res);
}
