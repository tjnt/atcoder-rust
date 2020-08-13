use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        m: u32
    }
    let m = match m {
        0...99        => 0,
        100...5000    => m * 10,
        6000...30000  => m + 50000,
        35000...70000 => (m - 30000) / 5 + 80000,
        _ => 89000
    };
    println!("{:02}", m / 1000);
}
