use proconio::{input, fastout};

pub fn divisor(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    for i in (1..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i)
            }
        }
    }
    res.sort();
    res
}

pub fn print_vec<T: std::string::ToString>(v: &[T], s: &str) {
    println!(
        "{}",
        v.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(s)
    );
}

#[fastout]
fn main() {
    input! {
        n: u64
    }
    print_vec(&divisor(n), "\n");
}
