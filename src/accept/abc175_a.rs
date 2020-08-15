use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let res = match &s[0..=2] {
        "SSS" => 0,
        "RSS" => 1,
        "SRS" => 1,
        "SSR" => 1,
        "RSR" => 1,
        "RRS" => 2,
        "SRR" => 2,
        "RRR" => 3,
        _ => panic!()
    };
    println!("{}", res);
}
