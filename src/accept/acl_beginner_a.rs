use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        k: usize
    }
    println!("{}", "ACL".repeat(k));
}
