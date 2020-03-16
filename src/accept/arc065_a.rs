use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let mut s: String = read();
    s = s.chars().rev().collect::<String>();
    let v = vec!("dream", "dreamer", "erase", "eraser");
    let v:Vec<String> = v.into_iter().map(|p| {
        p.chars().rev().collect::<String>()
    }).collect();

    while !s.is_empty() {
        let found = v.iter().find(|&z| { s.starts_with(z) } );
        match found {
            Some(z) => s = s.trim_left_matches(z).to_string(),
            None => {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
