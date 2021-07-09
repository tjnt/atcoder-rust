use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: [u32;4]
    }
    println!("{}", a.iter().min().unwrap());
}
