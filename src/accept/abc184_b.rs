use proconio::{input, fastout, };
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        x: i64,
        s: Chars
    }
    let res = s.iter().fold(x, |a, &c| {
        (if c == 'o' { a + 1 } else { a - 1 }).max(0)
    });
    println!("{}", res);
}
