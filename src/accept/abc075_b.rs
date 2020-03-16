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
        a: [chars; h],
    }
    let adjacent = |i: i32, j: i32| {
        [ (i+1,j), (i+1,j-1), (i,j-1), (i-1,j-1),
          (i-1,j), (i-1,j+1), (i,j+1), (i+1,j+1)] };
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                print!("#");
            } else {
                let ads = adjacent(i as i32,j as i32);
                let sum: usize = ads.iter().map(|&(ii,jj)|{
                    if 0 <= ii && ii < (h as i32) &&
                       0 <= jj && jj < (w as i32) {
                        if a[ii as usize][jj as usize] == '#' {
                            1
                        } else { 0 }
                    } else { 0 }
                }).sum();
                print!("{}", sum);
            }
        }
        println!();
    }
}
