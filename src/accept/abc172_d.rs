use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut res = 0;
    for i in 1..=n {
        let g = n / i;
        res += (g * (g+1) / 2) * i;
    }
    println!("{}", res);
}
