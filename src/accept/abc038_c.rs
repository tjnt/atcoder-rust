use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64;n]
    }
    let mut sum = 0;
    let mut x = 0;
    for i in 1usize..n {
        if a[i-1] < a[i] {
            x += 1;
            sum += x;
        } else {
            x = 0;
        }
    }
    println!("{}", sum + (n as u64));
}
