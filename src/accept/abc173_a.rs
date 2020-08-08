use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    if n % 1000 == 0 {
        println!("0");
    } else {
        println!("{}", 1000 - n % 1000);
    }
}
