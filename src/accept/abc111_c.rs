macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::HashMap;
fn count_elem<T>(v: &[T]) -> HashMap<&T, u64>
  where T: std::cmp::Eq + std::hash::Hash {
    let mut m = HashMap::new();
    for e in v {
        *m.entry(e).or_insert(0) += 1;
    }
    m
}

fn main() {
    input! {
        n: usize,
        vv: [u64;n]
    }
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for (i,v) in vv.iter().enumerate() {
        if i % 2 == 0 {
            even.push(*v);
        } else {
            odd.push(*v);
        }
    }
    let meven = count_elem(&even);
    let modd = count_elem(&odd);
    if vv[0] == vv[1] && meven.len() == 1 && modd.len() == 1 {
        println!("{}", vv.len() / 2);
        return;
    }
    fn revsort(m: &HashMap<&u64,u64>) -> Vec<(u64,u64)> {
        let mut res: Vec<(u64,u64)> =
            m.into_iter().map(|(k,v)| (**k,*v)).collect();
        res.sort_by_key(|t| t.1);
        res.reverse();
        res
    }
    let even = revsort(&meven);
    let odd = revsort(&modd);
    if even[0].0 != odd[0].0 {
        let sum = (n as u64) - even[0].1 - odd[0].1;
        println!("{}", sum);
    } else {
        let sum1 = (n as u64) - even[0].1 - odd[1].1;
        let sum2 = (n as u64) - odd[0].1 - even[1].1;
        println!("{}", std::cmp::min(sum1,sum2));
    }
}
