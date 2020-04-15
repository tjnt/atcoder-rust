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

pub struct Sieve {
    primes: Vec<usize>,
    tbl: Vec<usize>,
}
impl Sieve {
    pub fn new(n: usize) -> Sieve {
        let sz: usize = (n + 1) as usize;
        let mut primes = Vec::new();
        let mut tbl = vec![0; sz];
        tbl[0] = 0;
        tbl[1] = 0;
        for i in 2..sz {
            if tbl[i] != 0 {
                continue;
            }
            primes.push(i as usize);
            tbl[i] = i as usize;
            let mut j = i * i;
            while j < sz {
                if tbl[j] == 0 {
                    tbl[j] = i as usize;
                }
                j += i;
            }
        }
        Sieve { primes: primes, tbl: tbl }
    }
    pub fn primes(&self) -> &Vec<usize> {
        &self.primes
    }
    pub fn is_prime(&self, x: usize) -> bool {
        x > 1 && self.tbl[x as usize] == x
    }
    pub fn factorization(&self, x: usize) -> Vec<(usize, usize)> {
        let mut mx = x;
        let mut fl = Vec::new();
        while mx > 1 {
            let fx = self.tbl[mx as usize];
            fl.push(fx);
            mx /= fx;
        }
        if fl.is_empty() {
            return Vec::new();
        }
        let mut res = vec![(fl[0], 0)];
        for p in &fl {
            let l = res.len() - 1;
            let last = res[l];
            if last.0 == *p {
                res[l].1 += 1;
            } else {
                res.push((*p, 1));
            }
        }
        res
    }
}

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize
    }
    let s = Sieve::new(n);
    let mut tbl = vec![0;n+1];
    for i in 1..n+1 {
        let f = s.factorization(i);
        for x in f {
            tbl[x.0] += x.1;
        }
    }
    let mut res: u64 = 1;
    for v in tbl {
        res *= (v+1) as u64;
        res %= MOD;
    }
    println!("{}", res);
}
