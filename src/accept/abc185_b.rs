use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: i64,
        m: usize,
        t: i64,
        ab: [(i64,i64);m]
    }
    let mut x = n;
    let mut p = 0;
    for (a,b) in ab {
        x -= a - p;
        if x <= 0 {
            break;
        }
        x = n.min(x + b - a);
        p = b;
    }
    x -= t - p;
    println!("{}", if x > 0 {"Yes"} else {"No"});
}
