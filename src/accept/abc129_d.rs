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
        h: usize,
        w: usize,
        ss: [chars;h]
    }
    let mut tt = vec![vec![0;w];h];
    for i in 0..h {
        let mut l = 0;
        let mut r = 0;
        for j in 0..w {
            if ss[i][j] == '#' {
                l = 0;
            } else {
                l += 1;
                tt[i][j] += l;
            }
            let rj = w-j-1;
            if ss[i][rj] == '#' {
                r = 0;
            } else {
                r += 1;
                tt[i][rj] += r;
            }
        }
    }
    for j in 0..w {
        let mut l = 0;
        let mut r = 0;
        for i in 0..h {
            if ss[i][j] == '#' {
                l = 0;
            } else {
                l += 1;
                tt[i][j] += l;
            }
            let ri = h-i-1;
            if ss[ri][j] == '#' {
                r = 0;
            } else {
                r += 1;
                tt[ri][j] += r;
            }
        }
    }
    let mut max = 0;
    for l in tt {
        for c in l {
            max = std::cmp::max(max, c);
        }
    }
    println!("{}", max - 3);
}
/* vim:set foldmethod=marker: */
