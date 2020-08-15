use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let h: HashSet<char> = "atcoder".chars().collect();
    let res = s.iter().zip(t.iter()).all(|(&i,&j)| {
        i == j ||
        (i == '@' && h.contains(&j)) ||
        (j == '@' && h.contains(&i))
    });
    if res {
        println!("You can win");
    } else {
        println!("You will lose");
    }
}
