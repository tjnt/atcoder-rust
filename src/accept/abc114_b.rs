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

pub fn array_to_num(a: &[u32]) -> u32 {
    let mut n = 0;
    let mut d = u32::pow(10, (a.len() - 1) as u32) as u32;
    for i in a {
        n += i * d;
        d /= 10;
    }
    n
}

fn main() {
    input! {
        ss: chars
    }
    let mut nums = Vec::new();
    for i in 2..ss.len() {
        let ns: Vec<u32> = ss[i-2..i+1].iter()
            .map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        nums.push(array_to_num(&ns) as i64);
    }
    println!("{}",
        nums.into_iter().map(|n| (753-n).abs()).min().unwrap());
}
