use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let v = vec![
        a*c,
        a*d,
        b*c,
        b*d
    ];
    println!("{}", v.iter().max().unwrap());
}
