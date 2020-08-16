use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        t: u32,
        n: usize,
        aa: [u32;n],
        m: usize,
        bb: [u32;m]
    }
    let mut aa = aa;
    let mut cnt = 0;
    for b in bb {
        for a in aa.iter_mut() {
            if *a != 0 && *a <= b && b <= *a+t {
                *a = 0;
                cnt += 1;
                break;
            }
        }
    }
    println!("{}", if cnt == m { "yes" } else { "no" });
}
