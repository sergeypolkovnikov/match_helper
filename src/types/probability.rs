use std::ops::{Add, Mul};
use std::iter::Sum;
use std::iter::Iterator;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Probability(f64);

impl Probability {
    pub fn rev(self) -> Probability {
        Probability(1_f64 - self.0)
    }

    pub fn zero() -> Probability {
        Probability(0.0)
    }
}

impl From<f64> for Probability {
    fn from(val: f64) -> Probability {
        #[cfg(debug_assertions)]
        if val > 1.0 { panic!("Probability can't be greater 1 but reaches {}", val); }
        Probability(val)
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

impl Add for Probability{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let result = self.0 + other.0;
        #[cfg(debug_assertions)]
        if result > 1.0 { panic!("Probability can't be greater 1 but reaches {} + {} = {}", self.0, other.0, result); }
        Self(result)
    }
}

impl<'a> Sum<&'a Self> for Probability {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::zero(), |a, &b| a + b)
    }
}

impl Mul for Probability{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

