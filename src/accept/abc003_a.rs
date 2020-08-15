use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let f = n as f64;
    let mut res = 0.0;
    for i in 1..=n {
        res += ((i * 10000) as f64) / f;
    }
    println!("{}", res);
}
