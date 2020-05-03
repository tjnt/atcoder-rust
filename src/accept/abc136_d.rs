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
    let n = s.len();
    let mut res = vec![0;n];
    let mut l = 0;
    for i in 0..n {
        l = std::cmp::max(l,i);
        if s[i] == 'R' {
            while s[l] != 'L' {
                l += 1;
            }
            let idx = if i % 2 != l % 2 { l - 1 } else { l };
            res[idx] += 1;
        }
    }
    let mut r = n-1;
    for i in (0..n).rev() {
        r = std::cmp::min(i,r);
        if s[i] == 'L' {
            while s[r] != 'R' {
                r -= 1;
            }
            let idx = if i % 2 != r % 2 { r + 1 } else { r };
            res[idx] += 1;
        }
    }
    for r in res {
        print!("{} ", r);
    }
}
