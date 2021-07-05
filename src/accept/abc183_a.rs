use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i32
    }
    println!("{}", x.max(0));
}
