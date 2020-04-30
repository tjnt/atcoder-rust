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
        aa: [u32;n]
    }
    let mut bb = vec![0;n];
    for i in (1..n+1).rev() {
        let mut c = 0;
        let mut j = i;
        while j <= n {
            if i % j != 0 {
                c += bb[j-1];
            }
            j += i;
        }
        if c % 2 == aa[i-1] {
            bb[i-1] = 0;
        } else {
            bb[i-1] = 1;
        }
    }
    println!("{}", bb.iter().filter(|i| **i == 1).count());
    // println!("{:?}", bb);
    for i in 0..n {
        if bb[i] == 1 {
            print!("{} ", i+1);
        }
    }
}
