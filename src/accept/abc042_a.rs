use std::io::stdin;

macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) => {{
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        buf.trim().parse::<$t>().unwrap();
    }};
    ($($t:ty),*) => {{
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let mut w = buf.trim().split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

fn main() {
    let (a,b,c) = read!(i32,i32,i32);
    match (a,b,c) {
        (5,5,7) | (5,7,5) | (7,5,5) => { println!("YES"); }
        _ => { println!("NO"); }
    };
}
