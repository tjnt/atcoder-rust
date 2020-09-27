use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    if (a <= c && c <= b) ||
       (c <= a && a <= d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
