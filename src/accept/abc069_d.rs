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

use std::collections::vec_deque::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        aa: [usize;n]
    }
    let mut q = VecDeque::new();
    for (i,a) in aa.iter().enumerate() {
        for _ in 0..*a {
            q.push_back(i+1);
        }
    }
    let mut b = true;
    let mut vv = vec![vec![0;w];h];
    for i in 0..h {
        for j in 0..w {
            let k = if b {j} else {w-j-1};
            vv[i][k] = q.pop_front().unwrap();
        }
        b = !b;
    }
    for v in vv {
        for x in v {
            print!("{} ", x);
        }
        println!();
    }
}
