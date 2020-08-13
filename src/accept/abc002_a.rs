use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
    }
    println!("{}", std::cmp::max(x,y));
}
