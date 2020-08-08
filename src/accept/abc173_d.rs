use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64;n]
    }
    a.sort();
    a.reverse();
    let mut res = a[0];
    let mut i = 1;
    while i < n/2 {
        res += a[i] * 2;
        i += 1;
    }
    if n % 2 == 1 {
        res += a[i];
    }
    println!("{}", res);
}
