use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let n = n % 30;
    let mut v: Vec<usize> = vec![1,2,3,4,5,6];
    for i in 0..n {
        v.swap(i % 5, i % 5 + 1);
    }
    for i in v {
        print!("{}", i);
    }
    println!();
}
