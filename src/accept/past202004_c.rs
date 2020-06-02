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
        mat: [chars;n]
    }
    let mut mat = mat;
    for i in (0..n-1).rev() {
        for j in 1..(2*n-2) {
            if mat[i][j] == '#' {
                if mat[i+1][j-1] == 'X' ||
                   mat[i+1][j] == 'X'   ||
                   mat[i+1][j+1] == 'X' {
                    mat[i][j] = 'X'
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..(2*n-1) {
            print!("{}", mat[i][j]);
        }
        println!();
    }
}
/* vim:set foldmethod=marker: */
