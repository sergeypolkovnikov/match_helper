use std::ops::{Sub, Neg};
use serde::Deserialize;
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug, Ord, Deserialize, Display)]
pub struct Rating(i16);

impl Sub for Rating {
    type Output = i16;

    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

impl Neg for Rating {
    type Output = Rating;

    fn neg(self) -> Self::Output {
        Rating(-self.0)
    }
}
