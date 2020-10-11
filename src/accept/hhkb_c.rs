use proconio::{input, fastout};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        pp: [u64;n]
    }
    let mut m = 0;
    let mut h = HashSet::new();
    for p in pp {
        h.insert(p);
        while h.contains(&m) {
            m += 1;
        }
        println!("{}", m);
    }
}
