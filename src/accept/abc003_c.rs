use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut rr: [u64;n]
    }
    rr.sort();
    rr.reverse();
    rr = rr.into_iter().take(k).collect();
    rr.reverse();
    let res = rr.iter().map(|r| *r as f64)
                .fold(0.0, |c, r| (c + r) / 2.0 );
    println!("{}", res);
}
