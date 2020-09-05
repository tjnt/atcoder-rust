use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u32,
        x: u32,
        t: u32
    }
    if n % x == 0 {
        println!("{}", (n / x) * t);
    } else {
        println!("{}", (n / x + 1) * t);
    }
}
