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
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32,
    }
    let tx = tx - sx;
    let ty = ty - sy;
    let mut v = Vec::<char>::new();
    for _ in 0..ty { v.push('U'); }
    for _ in 0..tx { v.push('R'); }
    for _ in 0..ty { v.push('D'); }
    for _ in 0..tx+1 { v.push('L'); }
    for _ in 0..ty+1 { v.push('U'); }
    for _ in 0..tx+1 { v.push('R'); }
    v.push('D');
    v.push('R');
    for _ in 0..ty+1 { v.push('D'); }
    for _ in 0..tx+1 { v.push('L'); }
    v.push('U');
    println!("{}",
        v.into_iter().collect::<String>());
}
