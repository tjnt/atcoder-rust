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

fn main() {
    input! {
        n: usize,
        a: [u32;n]
    }
    let mut a = a;
    a.sort();
    let mut i;
    let mut k;
    if n % 2 == 0 {
        i = 0;
        k = 1;
    } else {
        if a[0] != 0 {
            println!("0");
            return;
        }
        i = 1;
        k = 2;
    }
    while i < n {
        if k != a[i] || k != a[i+1] {
            println!("0");
            return;
        }
        i += 2;
        k += 2;
    }
    println!("{}", ModInt(2).pow(n/2).value());
}
