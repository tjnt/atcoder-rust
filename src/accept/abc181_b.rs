use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [(u64, u64); n]
    }
    let mut sum = 0;
    for (a,b) in x {
        sum += (a+b) * (b-a+1) / 2;
    }
    println!("{}", sum);
}
