use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64;n]
    }
    for i in k..n {
        if a[i] > a[i-k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
