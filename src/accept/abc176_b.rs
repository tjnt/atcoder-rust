use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        nn: Chars
    }
    let mut x: u64  = 0;
    for n in nn {
        x += n as u64 - 48;
    }
    if x % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
