/* {{{ */
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
/* }}} */

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
type Num = usize;
const MOD: Num = 1_000_000_007;
#[derive(Clone, Copy)]
pub struct ModInt<T: Copy + Clone>(T);
impl ModInt<Num> {
    pub fn value(self) -> Num {
        self.0
    }
    pub fn pow(self, e: usize) -> ModInt<Num> {
        let mut result = ModInt(1);
        let mut cur = self;
        let mut e = e;
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
            }
            e >>= 1;
            cur *= cur;
        }
        result
    }
    pub fn factorial(self) -> ModInt<Num> {
        let mut cur = ModInt(1);
        for i in 2..(self.0 + 1) {
            cur *= i;
        }
        cur
    }
}
impl Add<Num> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn add(self, mut rhs: Num) -> ModInt<Num> {
        if rhs >= MOD {
            rhs %= MOD
        }
        let mut t = rhs + self.0;
        if t >= MOD {
            t -= MOD
        }
        ModInt(t)
    }
}
impl Add<ModInt<Num>> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
        self + rhs.0
    }
}
impl AddAssign<Num> for ModInt<Num> {
    fn add_assign(&mut self, other: Num) {
        *self = *self + other;
    }
}
impl AddAssign<ModInt<Num>> for ModInt<Num> {
    fn add_assign(&mut self, other: ModInt<Num>) {
        *self = *self + other;
    }
}
impl Sub<Num> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn sub(self, mut rhs: Num) -> ModInt<Num> {
        if rhs >= MOD {
            rhs %= MOD
        }
        let value = if self.0 < rhs {
            self.0 + MOD - rhs
        } else {
            self.0 - rhs
        };
        ModInt(value)
    }
}
impl Sub<ModInt<Num>> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
        self - rhs.0
    }
}
impl SubAssign<Num> for ModInt<Num> {
    fn sub_assign(&mut self, other: Num) {
        *self = *self - other;
    }
}
impl SubAssign<ModInt<Num>> for ModInt<Num> {
    fn sub_assign(&mut self, other: ModInt<Num>) {
        *self = *self - other;
    }
}
impl Mul<Num> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn mul(self, mut rhs: Num) -> ModInt<Num> {
        if rhs >= MOD {
            rhs %= MOD
        }
        let t = (self.0 * rhs) % MOD;
        ModInt(t)
    }
}
impl Mul<ModInt<Num>> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
        self * rhs.0
    }
}
impl MulAssign<Num> for ModInt<Num> {
    fn mul_assign(&mut self, rhs: Num) {
        *self = *self * rhs;
    }
}
impl MulAssign<ModInt<Num>> for ModInt<Num> {
    fn mul_assign(&mut self, rhs: ModInt<Num>) {
        *self = *self * rhs;
    }
}
impl Div<Num> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn div(self, mut rhs: Num) -> ModInt<Num> {
        if rhs >= MOD {
            rhs %= MOD
        }
        self * ModInt(rhs).pow(MOD - 2)
    }
}
impl Div<ModInt<Num>> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
        self / rhs.0
    }
}
impl DivAssign<Num> for ModInt<Num> {
    fn div_assign(&mut self, rhs: Num) {
        *self = *self / rhs
    }
}
impl DivAssign<ModInt<Num>> for ModInt<Num> {
    fn div_assign(&mut self, rhs: ModInt<Num>) {
        *self = *self / rhs
    }
}

pub struct BiCoef {
    fact: Vec<ModInt<Num>>,
    inv:  Vec<ModInt<Num>>,
    finv: Vec<ModInt<Num>>
}

impl BiCoef {
    pub fn new(n: usize) -> BiCoef {
        let n = n + 1;
        let mut fact: Vec<ModInt<Num>> = vec![ModInt(1);n];
        let mut inv:  Vec<ModInt<Num>> = vec![ModInt(1);n];
        let mut finv: Vec<ModInt<Num>> = vec![ModInt(1);n];
        for i in 2..n {
            fact[i] = fact[i-1] * i;
            inv[i] = ModInt(MOD) - inv[MOD%i] * (MOD/i);
            finv[i] = finv[i-1] * inv[i];
        }
        BiCoef {
            fact: fact,
            inv:  inv,
            finv: finv,
        }
    }
    pub fn factorial(self, n: usize) -> ModInt<Num> {
        self.fact[n]
    }
    pub fn inverse(self, n: usize) -> ModInt<Num> {
        self.inv[n]
    }
    pub fn fact_inverse(self, n: usize) -> ModInt<Num> {
        self.finv[n]
    }
    pub fn combination(self, n: usize, r: usize) -> ModInt<Num> {
        self.fact[n] * self.finv[r] * self.finv[n-r]
    }
}

fn main() {
    input! {
        x: i64,
        y: i64,
    }
    let t = 2 * x - y;
    if t % 3 != 0 {
        println!("0");
        return;
    }
    let m = t / 3;
    let n = x - (2*m);
    if n < 0 || m < 0 {
        println!("0");
        return;
    }
    let n = n as usize;
    let m = m as usize;
    let bi = BiCoef::new(n+m);
    println!("{}", bi.combination(n+m,n).value());
}
/* vim:set foldmethod=marker: */
