use crate::sign::Sign;
use gcd::Gcd;
use std::ops;
use std::fmt;

#[derive(Debug)]
pub struct Fraction {
    pub sign: Sign,
    numerator: u64,
    denumerator:  u64
}

impl Fraction {
    pub fn new(numerator: u64, denumerator: u64, sign: Sign) -> Self {
        let this = Fraction {
            numerator: numerator,
            denumerator: denumerator,
            sign: sign,
        };
        
        this.reduce()
    }

    pub fn negate(self) -> Self {
        let new_sign = {
            if self.sign == Sign::Positive {
                Sign::Negative
            }
            else {
                Sign::Positive
            }
        };

        Fraction {
            sign: new_sign,
            numerator: self.numerator,
            denumerator: self.denumerator,
        }
    }

    #[inline]
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    #[inline]
    pub fn denumerator(&self) -> u64 {
        self.denumerator
    }

    fn reduce(self) -> Self {
        let gcd = self.numerator.gcd(self.denumerator);
        
        Fraction {
            sign: self.sign,
            numerator: self.numerator / gcd,
            denumerator: self.denumerator / gcd,
        }
    }
}

impl ops::Add<Self> for Fraction {
    type Output = Self;

    fn add(self, rhs: Fraction) -> Self::Output {
        let (numerator, sign) = {
            let self_leveled = self.numerator * rhs.denumerator;
            let right_leveled = rhs.numerator * self.denumerator;

            if self.sign == rhs.sign {
                (self_leveled + right_leveled, self.sign)
            }
            else {
                if self_leveled > right_leveled {
                    (self_leveled - right_leveled, self.sign)
                }
                else {
                    (right_leveled - self_leveled, rhs.sign)
                }
            }
        };

        Fraction::new(
            numerator,
            self.denumerator * rhs.denumerator,
            sign,
        )
    }
}

impl ops::Mul<Self> for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.numerator,
            self.denumerator * rhs.denumerator,
            self.sign * rhs.sign,
        )
    }
}

impl ops::Div<Self> for Fraction {
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denumerator,
            self.denumerator * rhs.numerator,
            self.sign * rhs.sign,
        )
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative_sign = {
            if self.sign == Sign::Negative {
                "-"
            }
            else {
                ""
            }
        };
        
        write!(
            f,
            "{sign}{numerator}/{denumerator}",
            sign = negative_sign,
            numerator = self.numerator,
            denumerator = self.denumerator,
        )
    }
}