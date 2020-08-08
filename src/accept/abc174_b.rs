use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xys: [(i64,i64);n]
    }
    let mut cnt = 0;
    for (x,y) in xys {
        let b = x.pow(2) + y.pow(2);
        if b <= d.pow(2) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
