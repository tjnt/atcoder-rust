use proconio::input;

fn main() {
    input! {
        x: [u32;5]
    }
    println!("{}", x.iter().position(|v| *v == 0).unwrap() + 1);
}
