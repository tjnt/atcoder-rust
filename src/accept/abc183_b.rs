use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64
    }
    let a = sy / (sy + gy);
    let x = (gx - sx) * a + sx;
    println!("{}", x);
}
