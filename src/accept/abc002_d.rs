use proconio::{input, fastout};
use proconio::marker::Usize1;
use std::collections::{HashSet, HashMap};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(Usize1,Usize1);m]
    }
    let mut g = vec![HashSet::new();n];
    for (x,y) in v {
        g[x].insert(y);
        g[y].insert(x);
    }

    let mut res = 0;
    for b in 1..(1u64 << n) {
        let mut h = HashMap::new();
        for (i, e) in g.iter().enumerate() {
            if b & (1 << i) != 0 {
                h.insert(i, e);
            }
        }
        let mut cnt = 0;
        for (i,s) in &h {
            let mut a = true;
            for j in h.keys() {
                if *i != *j && !s.contains(j) {
                    a = false;
                }
            }
            if a { cnt += 1; }
        }
        res = std::cmp::max(res, cnt);
    }
    println!("{}", res);
}
