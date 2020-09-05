use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut res = std::u32::MAX;
    for i in 0..s.len() - t.len() + 1 {
        let mut c = 0;
        for j in 0..t.len() {
            if s[i+j] != t[j] {
                c += 1;
            }
        }
        res = std::cmp::min(res, c);
    }
    println!("{}", res);
}
