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

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [u64;n],
        bcs: [(usize,u64);m]
    }
    let mut aa = aa;
    aa.sort();
    let mut bcs = bcs;
    bcs.sort_by_key(|x| x.1);
    let mut p = 0;
    for (b,c) in bcs.into_iter().rev() {
        let mut k = 0;
        for j in p..n {
            if aa[j] < c {
                aa[j] = c;
                k += 1;
            } else {
                break;
            }
            if k == b { break; }
        }
        p += k;
        if n <= p { break; }
    }
    // println!("{:?}", aa);
    println!("{}", aa.iter().sum::<u64>());
}
