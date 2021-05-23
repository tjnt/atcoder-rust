use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u32;n]
    }
    let mut aa = aa;
    aa.sort();
    aa.dedup();
    aa.reverse();
    println!("{}", aa[1]);
}
