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
        m: usize,
        x: u64,
        aa: [(u64,[u64;m]);n]
    }
    let check = |a: &[u64]| -> bool {
        a.iter().all(|v| x <= *v)
    };
    let assign = |a: &mut[u64], b: &[u64]|{
        for (l,r) in a.iter_mut().zip(b) {
            *l += *r;
        }
    };

    let mut res: u64 = std::u64::MAX;
    for b in 0..(1u64 << n) {
        let mut v = vec![0;m];
        let mut cc = 0;
        for (i, t) in aa.iter().enumerate() {
            let c = t.0;
            let a = &t.1;
            if b & (1 << i) != 0 {
                assign(&mut *v, &a);
                cc += c;
            }
        }
        if check(&v) {
            res = std::cmp::min(res, cc);
        }
    }
    if res == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
/* vim:set foldmethod=marker: */
