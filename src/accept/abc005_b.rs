use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
       n: usize,
       t: [u32;n]
    }
    println!("{}", t.iter().min().unwrap());
}
