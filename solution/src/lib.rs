// about to mod a bunch of util files

pub mod numeric {
    use std::ops::{Add, AddAssign, BitAnd, Div, Mul, ShrAssign, Sub};

    const MOD: i64 = 1000000007; // modulo 1e9 + 7

    #[derive(Debug, Clone, Copy)]
    pub struct LongLongModulo(pub i64);

    #[macro_export]
    macro_rules! nil {
        () => {
            LongLongModulo(-1)
        };
    }

    #[macro_export]
    macro_rules! one {
        () => {
            LongLongModulo(1)
        };
    }

    #[macro_export]
    macro_rules! zero {
        () => {
            LongLongModulo(0)
        };
    }

    #[macro_export]
    macro_rules! ll {
        ($x:expr) => {
            LongLongModulo($x)
        };
    }

    impl Add for LongLongModulo {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self((self.0 + rhs.0) % MOD)
        }
    }
    impl AddAssign for LongLongModulo {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl Mul for LongLongModulo {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self((self.0 * rhs.0) % MOD)
        }
    }

    impl Div for LongLongModulo {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Self(self.0 / rhs.0)
        }
    }

    impl ShrAssign<i64> for LongLongModulo {
        fn shr_assign(&mut self, rhs: i64) {
            self.0 = self.0 >> rhs
        }
    }

    impl PartialEq<i64> for LongLongModulo {
        fn eq(&self, other: &i64) -> bool {
            self.0.eq(other)
        }
    }

    impl PartialOrd<i64> for LongLongModulo {
        fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(other)
        }
    }

    impl PartialEq for LongLongModulo {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl Eq for LongLongModulo {}

    impl PartialOrd for LongLongModulo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for LongLongModulo {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl BitAnd<i64> for LongLongModulo {
        type Output = i64;
        fn bitand(self, rhs: i64) -> Self::Output {
            self.0 & rhs
        }
    }

    impl Sub for LongLongModulo {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self((self.0 - rhs.0) % MOD)
        }
    }
    impl Sub<i64> for LongLongModulo {
        type Output = Self;
        fn sub(self, rhs: i64) -> Self::Output {
            Self((self.0 - rhs) % MOD)
        }
    }

    impl From<i64> for LongLongModulo {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl TryInto<usize> for LongLongModulo {
        type Error = <i64 as TryInto<usize>>::Error;
        fn try_into(self) -> Result<usize, Self::Error> {
            self.0.try_into()
        }
    }

    pub type LL = LongLongModulo;

    pub fn pow(x: LL, n: LL) -> LL {
        let mut x = x;
        let mut n = n;
        let mut res = one!();
        while n > 0 {
            res = match n & 1 {
                1 => res * x,
                _ => res,
            };
            x = x * x;
            n >>= 1;
        }
        res
    }

    pub fn factorial(n: LL) -> LL {
        static mut FAC: Vec<LL> = vec![];
        let index: usize = n.try_into().expect("n >= 0");
        unsafe {
            match FAC[..] {
                [] => FAC.push(one!()),
                _ => (),
            }
            match FAC.get(index) {
                None | Some(nil!()) => {
                    FAC.resize(2 * index.min(usize::MAX), nil!());
                    FAC[index] = factorial(n - 1) * n;
                    FAC[index]
                }
                Some(&fact) => fact,
            }
        }
    }

    pub fn inv_factorial(n: LL) -> LL {
        pow(factorial(n), LongLongModulo(MOD - 2))
    }
}
