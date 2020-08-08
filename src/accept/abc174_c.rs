use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        k: usize
    }
    let mut a = vec![0;1000001];
    a[1] = 7 % k;
    if a[1] == 0 {
        println!("1");
        return;
    }
    for i in 2..=k {
        a[i] = (a[i-1] * 10 + 7) % k;
        if a[i] == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
