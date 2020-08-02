use proconio::input;
use std::collections::hash_set::HashSet;

fn main() {
    input! {
        x: i64,
        n: usize,
        pp: [i64;n]
    }
    let all: HashSet<i64> = (0..=101).collect();
    let sub: HashSet<i64> = pp.into_iter().collect();
    let mut pp: Vec<i64> = all.difference(&sub).cloned().collect();
    pp.sort();
    let res = pp.iter().min_by(|&a, &b| {
        (x-a).abs().cmp(&(x-b).abs())
    }).unwrap();
    println!("{}", res);
}
