use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        cc: [Chars;h]
    }
    let mut res = 0;
    for hb in 0..(1u64 << h) {
        for wb in 0..(1u64 << w) {
            let mut cnt = 0;
            for i in 0..h {
                for j in 0..w {
                    if hb & (1 << i) == 0 &&
                       wb & (1 << j) == 0 &&
                       cc[i][j] == '#' {
                       cnt += 1;
                    }
                }
            }
            if cnt == k {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
