use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64;n],
        q: usize,
        bcs: [(usize,usize);q]
    }
    let mut v = vec![0; 100001];
    for a in &aa {
        v[*a as usize] += 1;
    }
    let mut s: u64 = aa.iter().sum();
    for (b,c) in bcs {
        if b < c {
            s += (v[b] * (c-b)) as u64;
        } else if b > c {
            s -= (v[b] * (b-c)) as u64;
        }
        v[c] += v[b];
        v[b] = 0;
        println!("{}", s);
    }
}
