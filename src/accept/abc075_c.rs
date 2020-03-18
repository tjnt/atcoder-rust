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

fn g_dfs(g: &Vec<Vec<usize>>, v: &mut Vec<bool>,
         path: (usize, usize), exclude: (usize, usize)) -> bool {
    if path.0 == path.1 {
        return true;
    }
    for x in &g[path.0] {
        if path.0 == exclude.0 && *x == exclude.1 {
            continue;
        }
        if v[*x] {
            continue;
        }
        v[*x] = true;
        if g_dfs(g, v, (*x, path.1), exclude) {
            return true;
        }
    }
    return false;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize1,usize1);m]
    }
    let mut g = vec![Vec::new();n];
    for &(a,b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut res = 0;
    for &exclude in &ab {
        let mut reachable = true;
        for a in 0..n {
            for b in 0..n {
                let mut v = vec![false;n];
                if !g_dfs(&g, &mut v, (a,b), exclude) {
                    reachable = false;
                    break;
                }
            }
        }
        if reachable {
            res += 1;
        }
    }
    println!("{}", m - res);
}
