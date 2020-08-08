use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String;n]
    }
    let mut m: HashMap<&str,usize> =
        [
            ("AC", 0),
            ("WA", 0),
            ("TLE", 0),
            ("RE", 0),
        ].iter().cloned().collect();
    for s in &ss {
        m.entry(s).and_modify(|e| *e += 1);
    }
    println!("AC x {}",  m["AC"]);
    println!("WA x {}",  m["WA"]);
    println!("TLE x {}", m["TLE"]);
    println!("RE x {}",  m["RE"]);
}
