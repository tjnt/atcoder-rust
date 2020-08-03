use proconio::input;

const M: u64 = 26;

fn main() {
    input! {
        n: u64
    }
    let mut n = n;
    let mut v = Vec::new();
    while n > 0 {
        v.push((b'a' + ((n - 1) % M) as u8) as char);
        n = (n - 1) / M;
    }
    println!("{}", v.iter().rev().collect::<String>());
}
