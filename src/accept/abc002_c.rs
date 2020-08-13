use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: (f64,f64),
        b: (f64,f64),
        c: (f64,f64),
    }
    let b = (b.0 - a.0, b.1 - a.1);
    let c = (c.0 - a.0, c.1 - a.1);
    println!("{}", (b.0 * c.1 - b.1 * c.0).abs() / 2.0);
}
