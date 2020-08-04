use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64;n],
        b: [u64;m]
    }

    let mut a: Vec<u64> = a.iter().scan(0, |acc, v| {
        *acc += v;
        Some(*acc)
    }).filter(|v| *v <= k).collect();
    a.insert(0,0);

    let mut b: Vec<u64> = b.iter().scan(0, |acc, v| {
        *acc += v;
        Some(*acc)
    }).collect();
    b.insert(0,0);
    b.reverse();

    let mut cnt = 0;
    let mut j = 0;
    for i in 0..a.len() {
        while j < b.len() {
            if k >= a[i] + b[j] {
                cnt = std::cmp::max(cnt, i + (b.len() - j - 1));
                break;
            }
            j += 1;
        }
    }
    println!("{}", cnt);
}
