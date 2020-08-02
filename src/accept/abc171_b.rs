use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        pp: [u64;n]
    }
    let mut pp = pp;
    pp.sort();
    println!("{}",
        pp.iter().take(k).sum::<u64>());
}
