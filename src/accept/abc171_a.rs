use proconio::input;

fn main() {
    input! {
        a: char,
    }
    println!("{}", if a.is_lowercase() { "a" } else { "A" });
}
