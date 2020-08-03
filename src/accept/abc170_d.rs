use proconio::input;

const N: usize = 1000001;

fn main() {
    input! {
        n: usize,
        aa: [usize;n]
    }
    let mut bb = vec![0;N];
    for a in &aa {
        let mut i = *a;
        let b = bb[i] != 0;
        while i < N {
            bb[i] += 1;
            i += a;
            if b { break; }
        }
    }
    println!("{}", aa.iter().filter(|a|{ bb[**a] == 1 }).count());
}
