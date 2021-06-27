use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u64;n]
    }
    let mut res = 2;
    let mut m = 0;
    for k in 2..=*aa.iter().max().unwrap() {
        let c = aa.iter().fold(0, |acc, a| {
            if a % k == 0 { acc + 1 } else { acc }
        });
        if m < c {
            m = c;
            res = k
        }
    }
    println!("{}", res);
}
