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

fn distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(i,j)| {
        (i-j).powf(2.0)
    }).sum::<f64>().sqrt()
}

pub fn is_integer(f: f64) -> bool {
    f - f.floor() == 0.0
}

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[f64; d]; n],
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in i+1..n {
            let dis = distance(&x[i], &x[j]);
            if is_integer(dis) { cnt+=1 }
        }
    }
    println!("{}", cnt);
}
