use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        mut c: u32,
        k: u32
    }
    let mut i = 0;
    loop {
        if a < b && b < c {
            println!("Yes");
            return;
        }
        if i == k {
            break;
        }
        if b >= c {
            c *= 2;
        } else if a >= b {
            b *= 2;
        }
        i += 1;
    }
    println!("No");
}
