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
        aa: [[usize;3];3],
        n: usize,
        bb: [usize;n]
    }
    let mut v = vec![vec![false;3];3];
    for b in bb {
        for i in 0..3 {
            for j in 0..3 {
                if aa[i][j] == b {
                    v[i][j] = true;
                }
            }
        }
    }
    for i in 0..3 {
        let mut r = true;
        let mut c = true;
        for j in 0..3 {
            if !v[i][j] {
                r = false;
            }
            if !v[j][i] {
                c = false;
            }
        }
        if r || c {
            println!("Yes");
            return;
        }
    }
    if (v[0][0] && v[1][1] && v[2][2]) ||
       (v[0][2] && v[1][1] && v[2][0]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
