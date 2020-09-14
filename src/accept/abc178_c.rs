use proconio::{input, fastout};

const MOD: i64 = 1_000_000_007;
pub fn pow_mod(x: i64, n: i64) -> i64 {
    let mut x = x;
    let mut n = n;
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * x % MOD
        }
        x = x * x % MOD;
        n >>= 1;
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: i64
    }
    let a = pow_mod(10, n);
    let b = pow_mod(9, n);
    let c = pow_mod(8, n);
    let mut res = (a + c - b - b) % MOD;
    res = (res + MOD) % MOD;
    println!("{}", res);
}
