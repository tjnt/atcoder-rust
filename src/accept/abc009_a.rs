use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    println!("{}", n / 2 + n % 2);
}
