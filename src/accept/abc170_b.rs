use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32
    }
    for a in 0..=x {
        let b = x - a;
        if 2 * a + 4 * b == y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
