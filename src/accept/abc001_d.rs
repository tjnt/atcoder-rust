use proconio::{input, fastout};

pub fn array_to_num(a: &[u64]) -> u64 {
    let mut n = 0;
    let mut d = u32::pow(10, (a.len() - 1) as u32) as u64;
    for i in a {
        n += i * d;
        d /= 10;
    }
    n
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String;n],
    }
    let mut v = Vec::new();
    for s in ss {
        let h1: u32 = s[0..2].parse().unwrap();
        let m1: u32 = s[2..4].parse().unwrap();
        let mut h2: u32 = s[5..7].parse().unwrap();
        let mut m2: u32 = s[7..9].parse().unwrap();
        let m1 = m1 - (m1 % 5);
        m2 = m2 + (if m2 % 5 == 0 { 0 } else { 5 - (m2 % 5) });
        if m2 == 60 {
            h2 += 1;
            m2 = 0;
        }
        v.push((h1*100+m1, h2*100+m2));
    }
    v.sort();
    v.push((9999,9999));
    // for i in &v { println!("{:?}", i); }

    let mut s1 = &v[0].0;
    let mut e1 = &v[0].1;
    for (s2,e2) in v.iter().skip(1) {
        if *e1 >= *s2 {
            if e1 < e2 { e1 = e2 };
        } else {
            println!("{:04}-{:04}", s1, e1);
            s1 = s2;
            e1 = e2;
        }
    }
}
