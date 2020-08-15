use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut v: [u64;n]
    }
    v.sort();
    let mut res = 0;
    for i in 0..n {
        for j in i+1..n {
            if v[i] == v[j] { continue; }
            for k in j+1..n {
                if v[j] == v[k] { continue; }
                if v[i] + v[j] > v[k] {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}
