use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Digit(u8);

impl Digit {
    /// Add two digits and return the least significant digit of the sum and
    /// carry.
    pub fn plus(self, other: Digit) -> (Digit, Digit) {
        let mut sum = self.0 + other.0;
        let mut carry = 0;
        if sum >= 10 {
            sum -= 10;
            carry = 1;
        }
        (Digit(sum), Digit(carry))
    }

    /// Multiply two digits and return the least significant digit of the
    /// product and carry.
    pub fn times(self, other: Digit) -> (Digit, Digit) {
        let prod = self.0 * other.0;
        let digit = prod % 10;
        let carry = prod / 10;
        (Digit(digit), Digit(carry))
    }
}

impl From<u8> for Digit {
    fn from(value: u8) -> Self {
        if value >= 10 {
            panic!("{value} is not a digit");
        }
        Self(value)
    }
}

/// Number with an arbitrary number of digits
///
/// Digits are listed with the least significant first. For example:
/// `BigNum(vec![0, 0, 1])` is equivalent to `100`
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BigNum(Vec<Digit>);

impl BigNum {
    pub fn from_iter(iter: impl Iterator<Item = impl Into<Digit>>) -> Self {
        let mut num = Self(iter.map(Into::into).collect());
        num.trim_leading_zeros();
        num
    }

    pub fn digits(&self) -> usize {
        self.0.len()
    }

    pub fn zero() -> Self {
        Self(vec![Digit(0)])
    }

    pub fn one() -> Self {
        Self(vec![Digit(1)])
    }

    pub fn pow(self, exponent: usize) -> BigNum {
        if exponent == 0 {
            return BigNum::one();
        }
        let mut power = self.clone();
        for _ in 1..exponent {
            power = power * self.clone();
        }
        power
    }

    fn empty() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, digit: impl Into<Digit>) {
        self.0.push(digit.into());
    }

    fn pad_to(&mut self, length: usize) {
        for _ in 0..length - self.digits() {
            self.0.push(Digit(0));
        }
    }

    fn trim_leading_zeros(&mut self) {
        while self.0.last() == Some(&Digit(0)) {
            self.0.pop();
        }
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, digit) in self.0.iter().enumerate().rev() {
            write!(f, "{}", digit.0)?;
            if idx != 0 && idx % 3 == 0 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

impl Add for BigNum {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let length = self.digits().max(rhs.digits());
        self.pad_to(length);
        rhs.pad_to(length);

        let mut carry = Digit(0);

        for idx in 0..length {
            let (sum, carry1) = self.0[idx].plus(rhs.0[idx]);
            let (sum, carry2) = sum.plus(carry);
            self.0[idx] = sum;
            carry = carry1.max(carry2);
        }

        if carry == Digit(1) {
            self.push(1);
        }

        self.trim_leading_zeros();
        self
    }
}

impl Mul for BigNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut intermediates = Vec::with_capacity(rhs.digits());

        for i in 0..rhs.digits() {
            let mut intermediate = BigNum::empty();
            for _ in 0..i {
                intermediate.push(Digit(0));
            }
            let mut carry = Digit(0);

            for j in 0..self.digits() {
                let (digit, carry1) = rhs.0[i].times(self.0[j]);
                let (digit, carry2) = digit.plus(carry);
                intermediate.push(digit);
                // ???: This will never yield a carry, right?
                carry = carry1.plus(carry2).0;
            }

            if carry != Digit(0) {
                intermediate.push(carry);
            }

            intermediates.push(intermediate);
        }

        intermediates.into_iter().fold(BigNum::zero(), |a, b| a + b)
    }
}

impl From<u32> for BigNum {
    fn from(mut value: u32) -> Self {
        let mut digits = Vec::new();
        while value > 0 {
            digits.push(Digit((value % 10) as u8));
            value /= 10;
        }
        BigNum(digits)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_iter_removes_leading_zeros() {
        let num = BigNum::from_iter([0, 5, 1, 0, 0].into_iter());
        let expected = BigNum::from_iter([0, 5, 1].into_iter());
        assert_eq!(num, expected);
    }

    #[test]
    fn add_no_carry() {
        let a = BigNum::from_iter([3, 6].into_iter());
        let b = BigNum::from_iter([6, 3].into_iter());
        let expected = BigNum::from_iter([9, 9].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_with_carry() {
        let a = BigNum::from_iter([4, 6].into_iter());
        let b = BigNum::from_iter([6, 3].into_iter());
        let expected = BigNum::from_iter([0, 0, 1].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_more_digits_lhs() {
        let a = BigNum::from_iter([2, 9, 4, 5].into_iter());
        let b = BigNum::from_iter([9, 9].into_iter());
        let expected = BigNum::from_iter([1, 9, 5, 5].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_more_digits_rhs() {
        let a = BigNum::from_iter([7, 8, 5].into_iter());
        let b = BigNum::from_iter([4, 7, 0, 3, 9].into_iter());
        let expected = BigNum::from_iter([1, 6, 6, 3, 9].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn mul() {
        let a = BigNum::from_iter([8, 3, 6, 7].into_iter());
        let b = BigNum::from_iter([4, 3, 2].into_iter());
        let expected = BigNum::from_iter([2, 9, 2, 7, 8, 7, 1].into_iter());
        assert_eq!(a * b, expected);
    }

    #[test]
    fn pow_zero() {
        let a = BigNum::from_iter([2].into_iter());
        let e = 0;
        let expected = BigNum::from_iter([1].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_one() {
        let a = BigNum::from_iter([2].into_iter());
        let e = 1;
        let expected = BigNum::from_iter([2].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_two() {
        let a = BigNum::from_iter([2].into_iter());
        let e = 2;
        let expected = BigNum::from_iter([4].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_big() {
        let a = BigNum::from_iter([2].into_iter());
        let e = 30;
        let expected = BigNum::from_iter([4, 2, 8, 1, 4, 7, 3, 7, 0, 1].into_iter());
        assert_eq!(a.pow(e), expected);
    }
}
