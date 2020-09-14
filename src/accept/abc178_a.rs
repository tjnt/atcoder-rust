use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: u32
    }
    println!("{}", if x == 0 {1} else {0});
}
