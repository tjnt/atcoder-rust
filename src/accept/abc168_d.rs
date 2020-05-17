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

use std::cell::RefCell;
pub struct GraphL {
    g: Vec<Vec<usize>>,
    res: RefCell<Vec<usize>>,
}
impl GraphL {
    pub fn bfs(&self, s: usize) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        // self.res.borrow_mut().push(s);
        while let Some(&u) = queue.front() {
            for v in self.g[u].iter() {
                if self.res.borrow_mut()[*v] != std::usize::MAX {
                    continue;
                }
                self.res.borrow_mut()[*v] = u;
                queue.push_back(*v);
            }
            queue.pop_front();
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize1,usize1);m]
    }
    let mut g = vec![Vec::new();n];
    for (l,r) in edges {
        g[l].push(r);
        g[r].push(l);
    }
    let graph = GraphL {
        g: g,
        res: RefCell::new(vec![std::usize::MAX;n])
    };
    graph.bfs(0);
    let res = graph.res.borrow();
    // println!("{:?}", res);
    println!("Yes");
    for i in res.iter().skip(1) {
        println!("{}", i+1);
    }
}
/* vim:set foldmethod=marker: */
