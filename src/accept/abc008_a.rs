use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: u32,
        t: u32
    }
    println!("{}", t - s + 1);
}
