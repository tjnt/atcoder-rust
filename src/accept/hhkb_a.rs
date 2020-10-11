use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String
    }
    if s == "Y" {
        println!("{}", t.to_uppercase());
    } else {
        println!("{}", t);
    }
}
