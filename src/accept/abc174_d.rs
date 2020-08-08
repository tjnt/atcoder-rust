use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut c: Chars
    }
    let mut cnt = 0;
    let mut j = n-1;
    for i in 0..n {
        if i >= j { break; }
        if c[i] == 'W' {
            while i < j {
                if c[j] == 'R' {
                    c[i] = 'R';
                    c[j] = 'W';
                    cnt += 1;
                    break;
                }
                j -= 1;
            }
        }
    }
    println!("{}", cnt);
}
