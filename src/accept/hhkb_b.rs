use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mat: [Chars; h],
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if mat[i][j] != '#' {
                if j+1 < w && mat[i][j+1] == '.' {
                    res += 1;
                }
                if i+1 < h && mat[i+1][j] == '.' {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}
