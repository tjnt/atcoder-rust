use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64;n]
    }
    let mut y = 1000;
    for i in 0..n-1 {
        if a[i] < a[i+1] {
            let k = y / a[i];
            y += (a[i+1] - a[i]) * k;
        }
    }
    println!("{}", y);
}
