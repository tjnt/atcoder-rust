use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut v = vec![0u64;1000001];
    v[3] = 1;
    for i in 4..=n {
        v[i] = (v[i-1] + v[i-2] + v[i-3]) % 10007;
    }
    println!("{}", v[n]);
}
