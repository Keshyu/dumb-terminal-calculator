use std::ops;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

impl ops::Mul<Self> for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            self
        }
        else {
            Sign::Negative
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Sign::*;

        match self {
            Positive => write!(f, "+"),
            Negative => write!(f, "-"),
        }
    }
}