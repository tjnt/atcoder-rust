use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String;n]
    }
    let mut h = HashMap::new();
    for s in ss {
        *h.entry(s).or_insert(0) += 1;
    }
    println!("{}", h.iter().max_by_key(|t| t.1).unwrap().0);
}
