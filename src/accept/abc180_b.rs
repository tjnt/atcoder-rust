use proconio::{input, fastout};

fn f1(xx: &[i64]) -> i64 {
    xx.iter().map(|x| x.abs()).sum()
}

fn f2(xx: &[i64]) -> f64 {
    (xx.iter().map(|x| x*x).sum::<i64>() as f64).sqrt()
}

fn f3(xx: &[i64]) -> i64 {
    xx.iter().map(|x| x.abs()).max().unwrap()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xx: [i64;n]
    }
    println!("{}", f1(&xx));
    println!("{}", f2(&xx));
    println!("{}", f3(&xx));
}
