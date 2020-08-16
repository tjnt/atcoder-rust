use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut n: u64,
        mut m: u64
    }
    let b;
    if m % 2 == 0 {
        b = 0;
    } else {
        b = 1;
        n -= 1;
    }
    for a in 0..=n {
        let c = n - a;
        if 2*a + 3*b + 4*c == m {
            println!("{} {} {}", a, b, c);
            return;
        }
    }
    println!("-1 -1 -1");
}
