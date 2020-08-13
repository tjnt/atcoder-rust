use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        w: Chars
    }
    for c in w {
        match c {
            'a'|'i'|'u'|'e'|'o' => (),
            _ => print!("{}", c)
        };
    }
    println!();
}
