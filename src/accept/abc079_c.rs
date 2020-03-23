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
        s: chars
    }
    let s: Vec<i32> = s.iter().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let n = s.len();
    let mut b: u64 = 1;
    while b < (1u64 << n) {
        let mut sum = s[0];
        for (i, &v) in s.iter().skip(1).enumerate() {
            if b & (1 << i) != 0 {
                sum += v;
            } else {
                sum -= v;
            }
        }
        if sum == 7 {
            break;
        }
        b += 1;
    }
    for i in 0..n-1 {
        print!("{}", s[i]);
        if b & (1 << i) != 0 {
            print!("+");
        } else {
            print!("-");
        }
    }
    println!("{}=7", s[n-1]);
}
