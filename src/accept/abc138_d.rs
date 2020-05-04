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
        q: usize,
        ab: [(usize1,usize1);n-1],
        px: [(usize1,u64);q]
    }
    let mut g = vec![Vec::new();n];
    for (a,b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut px1 = vec![0;n];
    for (p,x) in px {
        px1[p] += x;
    }

    let mut res = vec![0;n];
    let mut vis = vec![false;n];

    struct Recurse<'s> {
        go: &'s Fn(&Recurse, usize, u64, &mut Vec<bool>, &mut Vec<u64>)
    }
    let dfs = Recurse {
        go: &|f, u, x, vis, res| {
            if vis[u] { return; }
            vis[u] = true;
            res[u] += px1[u] + x;
            for v in &g[u] {
                (f.go)(&f, *v, res[u], vis, res)
            }
        }
    };
    (dfs.go)(&dfs, 0, 0, &mut vis, &mut res);
    for v in res {
        print!("{} ", v);
    }
}
/* vim:set foldmethod=marker: */
