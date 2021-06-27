use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64
    }
    println!("{}", (2 * a) + 100 - b);
}
