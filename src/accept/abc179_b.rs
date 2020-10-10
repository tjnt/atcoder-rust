use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        v: [(u32,u32);n]
    }
    let mut cnt = 0;
    for (a,b) in v {
        if a == b {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == 3 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
