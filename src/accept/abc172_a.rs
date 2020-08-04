use proconio::input;

fn main() {
    input! {
        a: u64
    }
    println!("{}", a + a.pow(2) + a.pow(3));
}
