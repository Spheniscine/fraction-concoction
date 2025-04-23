use serde::{Deserialize, Serialize};

use super::CommonNumExt;
use std::{cmp::Ordering, fmt::Write, ops::{Add, Sub, AddAssign, SubAssign}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fraction {
    n: i64, d: i64
}

impl From<i64> for Fraction {
    fn from(value: i64) -> Self {
        Self { n: value, d: 1 }
    }
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Self { n: numerator, d: denominator }.reduce()
    }

    fn reduce(mut self) -> Self {
        let mut g = self.n.gcd(self.d);
        if self.d < 0 { g = -g; }
        self.n /= g; self.d /= g;
        self
    }

    pub fn zero() -> Self {
        Self::from(0)
    }

    pub fn one() -> Self {
        Self::from(1)
    }

    pub fn numerator(&self) -> i64 { self.n }
    pub fn denominator(&self) -> i64 { self.d }

    pub fn checked_add(self, rhs: impl Into<Self>) -> Option<Self> {
        let b: Self = rhs.into();
        let g = self.d.gcd(b.d);
        Some(Self::new(
            self.n.checked_mul(b.d / g)?.checked_add(b.n.checked_mul(self.d / g)?)?,
            self.d.checked_mul(b.d / g)?
        ))
    }

    pub fn checked_sub(self, rhs: impl Into<Self>) -> Option<Self> {
        let b: Self = rhs.into();
        let g = self.d.gcd(b.d);
        Some(Self::new(
            self.n.checked_mul(b.d / g)?.checked_sub(b.n.checked_mul(self.d / g)?)?,
            self.d.checked_mul(b.d / g)?
        ))
    }

    /// expresses fraction as (s, p, q, r), where self = s * (p + q / r), and s is the signum.
    pub fn mixed_fraction_parts(&self) -> (i8, i64, i64, i64) {
        let s = self.n.signum() as i8;
        let n = self.n.abs();
        (s, n / self.d, n % self.d, self.d)
    }

    pub fn to_tex(&self) -> String {
        let (s, p, q, r) = self.mixed_fraction_parts();
        let mut res = String::new();
        {
            let res = &mut res;
            if s < 0 { res.push('-'); }
            if q == 0 || p != 0 {
                write!(res, "{}", p).ok();
            }
            if q != 0 {
                write!(res, "\\large\\frac{{{}}}{{{}}}", q, r).ok();
            }
        }
        res
    }
}

impl <V: Into<Self>> Add<V> for Fraction {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.checked_add(rhs).unwrap()
    }
}

impl <V: Into<Self>> Sub<V> for Fraction {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        self.checked_sub(rhs).unwrap()
    }
}

impl <V: Into<Self>> AddAssign<V> for Fraction {
    #[inline]
    fn add_assign(&mut self, rhs: V) { *self = *self + rhs; }
}
impl <V: Into<Self>> SubAssign<V> for Fraction {
    #[inline]
    fn sub_assign(&mut self, rhs: V) { *self = *self - rhs; }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.n.checked_mul(other.d)?.cmp(&(other.n.checked_mul(self.d)?)))
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::zero()
    }
}