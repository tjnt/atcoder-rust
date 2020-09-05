use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u64;n]
    }
    let mut m = aa[0];
    let mut res = 0;
    for a in aa.iter().skip(1) {
        if *a < m {
            res += m - a;
        } else if *a > m {
            m = *a;
        }
    }
    println!("{}", res);
}
