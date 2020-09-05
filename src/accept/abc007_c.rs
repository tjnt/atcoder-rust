use proconio::{input, fastout};
use proconio::marker::{Chars,Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        r: usize,
        _c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        mut mz: [Chars;r]
    }
    let mut q = VecDeque::<(usize,usize)>::new();
    let mut vv = vec![vec![None;mz[0].len()];mz.len()];
    let next = |t: (usize, usize)| {
        vec![(t.0+1, t.1), (t.0, t.1+1),
             (t.0-1, t.1), (t.0, t.1-1)]
    };
    q.push_back(s);
    vv[s.0][s.1] = Some(0);
    while let Some(&u) = q.front() {
        if u == g {
            println!("{}", vv[u.0][u.1].unwrap());
        }
        for v in next(u) {
            if vv[v.0][v.1].is_none() && mz[v.0][v.1] == '.' {
                q.push_back(v);
                vv[v.0][v.1] = Some(vv[u.0][u.1].unwrap() + 1);
            }
        }
        q.pop_front();
    }
}
