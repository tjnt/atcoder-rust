use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", if n % 2 == 0 { "White" } else { "Black" });
}
