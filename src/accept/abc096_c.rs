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
        h: usize,
        w: usize,
        ss: [chars; h],
    }
    let mut ss = ss;
    for i in 0..h {
        ss[i].insert(0,'.');
        ss[i].push('.');
    }
    ss.insert(0,vec!['.';w+2]);
    ss.push(vec!['.';w+2]);
    let mut res = true;
    for i in 1..h+1 {
        for j in 1..w+1 {
            if ss[i][j] == '#' {
                if ss[i-1][j] == '.' &&
                   ss[i+1][j] == '.' &&
                   ss[i][j-1] == '.' &&
                   ss[i][j+1] == '.' {
                       res = false;
                }
            }
        }
    }
    println!("{}", if res {"Yes"} else {"No"});
}
