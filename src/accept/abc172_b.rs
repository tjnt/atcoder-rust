use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{}",
        s.chars().zip(t.chars())
        .filter(|(a,b)|{ *a != *b }).count());
}
