use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u64,
        k: u64,
        a: [u64;n]
    }
    let mut s: Vec<u64> = a.iter().scan(0, |s, x|{
        *s += *x;
        Some(*s)
    }).collect();
    s.insert(0, 0);
    let mut res = 0;
    for i in k..=n {
        res += s[i as usize] - s[(i-k) as usize];
    }
    println!("{}", res);
}
