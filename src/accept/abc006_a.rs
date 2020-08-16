use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u32
    }
    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
