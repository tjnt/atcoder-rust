use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32;n]
    }
    println!("{}",
        a.iter().enumerate()
         .filter(|(i,v)| i % 2 == 0 && *v % 2 != 0).count());
}
