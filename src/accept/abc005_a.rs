use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: u32,
        y: u32
    }
    println!("{}", y / x);
}
