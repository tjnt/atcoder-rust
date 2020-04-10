#[doc = " use std::str::FromStr;"]
pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
   let b: String = read(); 
   let b = b.chars().next().unwrap();
   println!("{}",
       match b {
           'A' => 'T',
           'T' => 'A',
           'C' => 'G',
           'G' => 'C',
           _   => panic!()
       });
}
