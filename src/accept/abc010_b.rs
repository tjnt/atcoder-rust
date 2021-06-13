use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u32; n]
    }
    let mut res = 0;
    for a in aa {
        let mut a = a;
        while a % 2 == 0 || a % 3 == 2 {
            res += 1;
            a -= 1;
        }
    }
    println!("{}", res);
}
