use proconio::{input, fastout};

pub fn num_to_array(mut n: u128) -> Vec<u128> {
    let mut v = Vec::new();
    let d = 10;
    while n != 0 {
        v.push(n % d);
        n /= d;
    }
    v.reverse();
    v
}

const INF: u128 = std::u128::MAX;

#[fastout]
fn main() {
    input! {
        n: u128
    }
    let nn = num_to_array(n);
    let mut a = vec![0;3];
    for n in &nn {
        a[(n % 3) as usize] += 1
    }
    let mut res = INF;
    let x: u128 = nn.iter().sum();
    for i in 0..3 {
        for j in 0..3 {
            if i > a[1] { continue }
            if j > a[2] { continue }
            if (i + j) as usize == nn.len() { continue }
            let mut nx = x;
            nx -= i * 1;
            nx -= j * 2;
            if nx % 3 == 0 { res = res.min(i+j)}
        }
    }
    if res == INF {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
