use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: String
    }
    if a == "a" {
        println!("-1");
    } else {
        println!("a");
    }
}
