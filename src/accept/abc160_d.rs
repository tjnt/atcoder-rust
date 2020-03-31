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

use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: usize1,
        y: usize1,
    }
    let mut res: Vec<usize> = vec![0;n];
    for i in 0..n {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut d = vec![std::usize::MAX;n];
        q.push_front(i);
        d[i] = 0;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            let mut push = |v: usize| {
                if d[v] != std::usize::MAX { return; }
                d[v] = d[u] + 1;
                q.push_back(v);
            };
            if u > 0   { push(u-1); }
            if u < n-1 { push(u+1); }
            if u == x  { push(y); }
            if u == y  { push(x); }
        }
        for j in i..n {
            res[d[j]] += 1;
        }
    }
    for i in res.iter().skip(1) {
        println!("{}", i);
    }
}
