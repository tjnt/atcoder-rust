use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    let res = (1..=n).map(|a| (n-1)/a).sum::<u64>();
    println!("{}", res);
}
