use std::ops::Sub;
use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug, Ord, Deserialize)]
pub struct Rating(u16);

impl Sub for Rating{
    type Output = i16;

    fn sub(self, other: Self) -> Self::Output {
        self.0 as i16 - other.0 as i16
    }
}
