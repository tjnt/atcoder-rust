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
        k: usize
    }
    let mut q: VecDeque<i64> = VecDeque::new();
    for i in 1..10 {
        q.push_back(i);
    }
    if k <= 9 {
        println!("{}", q[k-1]);
        return;
    }
    let mut cnt = q.len();
    loop {
        let x = *q.front().unwrap();
        q.pop_front();
        for i in &[-1,0,1] {
            let d = x % 10 + i;
            if d < 0 || 9 < d { continue; }
            let nx = x * 10 + d;
            q.push_back(nx);
            cnt += 1;
            if cnt == k {
                println!("{}", q.back().unwrap());
                return;
            }
        }
    }
}
