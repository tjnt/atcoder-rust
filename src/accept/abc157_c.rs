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
        m: i32,
        sc: [(usize1,i32); m]
    }
    let mut buf: Vec<i32> = vec![-1;n];
    for (s,c) in sc {
        if n >= 2 && s == 0 && c == 0 {
            println!("-1");
            return;
        }
        if buf[s] == -1 || buf[s] == c {
            buf[s] = c;
        } else {
            println!("-1");
            return;
        }
    }
    let buf = buf.into_iter().map(|v| {
        if v == -1 { 0 } else { v }
    });
    let mut b = true;
    let buf = buf.map(|v| {
        if v == 0 {
            if b {
                b = false;
                if n == 1 { 0 } else { 1 }
            } else { 0 }
        } else {
            b = false;
            v
        }
    });
    for v in buf {
        print!("{}", v);
    }
}
