use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        l: u32,
        r: u32,
        d: u32
    }
    println!("{}",
        (l..=r).filter(|v| v % d == 0).count());
}
