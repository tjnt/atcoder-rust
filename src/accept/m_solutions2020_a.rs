use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: u32
    }
    let res = match x {
        400 ... 599  => 8,
        600 ... 799  => 7,
        800 ... 999  => 6,
        1000 ... 1199 => 5,
        1200 ... 1399 => 4,
        1400 ... 1599 => 3,
        1600 ... 1799 => 2,
        1800 ... 1999 => 1,
        _ => panic!()
    };
    println!("{}", res);
}
