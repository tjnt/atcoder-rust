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
        cc: [chars; h],
    }

    let mut s = (0,0);
    'outer: for i in 0..h {
        for j in 0..w {
            if cc[i][j] == 's' {
                s = (i,j);
                break 'outer;
            }
        }
    }

    let mut stack = Vec::new();
    let mut visit = vec![vec![false;w];h];
    let mut yes = false;
    stack.push(s);

    while let Some(&(i,j)) = stack.last() {
        // println!("{:?}", (i,j));
        visit[i][j] = true;
        if cc[i][j] == 'g' {
            yes = true;
            break;
        }

        let a = [(1,0),(0,1),(-1,0),(0,-1)];
        let mut next = a.iter().map(|&(y,x)| {
            (i as i32 + y, j as i32 + x)
        }).filter(|&(i,j)| {
            let ui = i as usize;
            let uj = j as usize;
            i >= 0 && j >= 0 && i < h as i32 && j < w as i32 &&
            cc[ui][uj] != '#' && !visit[ui][uj]
        }).map(|(i,j)| { (i as usize, j as usize) });

        if let Some(t) = next.next() {
            stack.push(t);
        } else {
            stack.pop();
        }
    }

    println!("{}", if yes {"Yes"} else {"No"});
}
/* vim:set foldmethod=marker: */
