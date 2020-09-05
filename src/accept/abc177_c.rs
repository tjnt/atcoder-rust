use proconio::{input, fastout};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt;

type Num = u64;
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
    pub fn inv(self) -> ModInt<Num> {
        self.pow((MOD - 2) as usize)
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
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Num> for ModInt<Num> {
    type Output = ModInt<Num>;
    fn div(self, mut rhs: Num) -> ModInt<Num> {
        if rhs >= MOD {
            rhs %= MOD
        }
        self * ModInt(rhs).inv()
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
impl fmt::Display for ModInt<Num> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for ModInt<Num> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModInt").field("value", &self.0).finish()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u64;n]
    }
    let v = aa.iter().scan(ModInt(0), |acc, a| {
        *acc += *a;
        Some(*acc)
    }).collect::<Vec<_>>();

    let mut res = ModInt(0);
    let l = v.iter().last().unwrap();
    for i in 0..v.len() {
        res += (*l - v[i]) * aa[i];
    }
    println!("{}", res);
}
