use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i64,
        k: u64,
        d: u64
    }
    let x = x.abs() as u64;
    if k <= x / d {
        println!("{}", x - k * d);
    } else {
        let kk = k - x / d;
        if kk % 2 == 0 {
            println!("{}", x % d);
        } else {
            println!("{}", d - x % d);
        }
    }
}
