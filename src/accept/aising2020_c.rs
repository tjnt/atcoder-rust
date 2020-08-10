use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut v = vec![0; n+1];
    let m = (n as f64).sqrt() as usize;
    for x in 1..m {
        for y in x..m {
            for z in y..m {
                let i = x*x + y*y + z*z + x*y + y*z + z*x;
                if i <= n {
                    if x == y && y == z {
                        v[i] += 1;
                    } else if x != y && y != z && x != z {
                        v[i] += 6;
                    } else {
                        v[i] += 3;
                    }
                }
            }
        }
    }
    for i in v.iter().skip(1) {
        println!("{}", i);
    }
}
