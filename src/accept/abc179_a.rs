use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String
    }
    if s.chars().last().unwrap() == 's' {
        println!("{}", s + "es");
    } else {
        println!("{}", s + "s");
    }
}
