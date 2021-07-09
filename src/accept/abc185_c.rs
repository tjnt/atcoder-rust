use proconio::{input, fastout};

pub fn ncr(n: u128, r: u128) -> u128 {
    (n - r + 1..n + 1).product::<u128>() / (1..r + 1).product::<u128>()
}

#[fastout]
fn main() {
    input! {
        l: u128
    }
    println!("{}", ncr(l-1, 11));
}
