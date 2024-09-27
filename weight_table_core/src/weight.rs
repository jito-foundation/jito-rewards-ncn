use std::u64;

use bytemuck::{Pod, Zeroable};
use jito_bytemuck::types::PodU64;
use shank::ShankType;

use crate::error::WeightTableError;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct Weight {
    numerator: PodU64,
    denominator: PodU64,
}

impl Weight {
    pub fn numerator(&self) -> u64 {
        self.numerator.into()
    }

    pub fn denominator(&self) -> u64 {
        self.denominator.into()
    }

    pub fn new(numerator: u64, denominator: u64) -> Result<Self, WeightTableError> {
        if denominator == 0 {
            return Err(WeightTableError::DenominatorIsZero);
        }

        Ok(Self {
            numerator: PodU64::from(numerator),
            denominator: PodU64::from(denominator),
        })
    }

    fn is_zero(&self) -> bool {
        self.numerator() == 0
    }

    fn gcd(&self) -> Result<u64, WeightTableError> {
        let mut n: u64 = self.numerator();
        let mut d: u64 = self.denominator();

        if d == 0 {
            return Err(WeightTableError::DenominatorIsZero);
        }

        if n == 0 {
            return Ok(1);
        }

        while d != 0 {
            if d < n {
                std::mem::swap(&mut d, &mut n);
            }
            d %= n;
        }

        Ok(n)
    }

    fn simplify(&self) -> Result<Self, WeightTableError> {
        let gcd_value = self.gcd()?;

        if gcd_value == 1 {
            return Ok(*self);
        }

        Ok(Self {
            numerator: PodU64::from(self.numerator() / gcd_value),
            denominator: PodU64::from(self.denominator() / gcd_value),
        })
    }

    fn compare_weights<F>(&self, other: &Self, compare: F) -> bool
    where
        F: Fn(u64, u64) -> bool,
    {
        let a = self.numerator();
        let b = self.denominator();
        let c = other.numerator();
        let d = other.denominator();

        a.checked_mul(d)
            .and_then(|ad| b.checked_mul(c).map(|bc| compare(ad, bc)))
            .unwrap_or(false)
    }

    pub fn gte(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad >= bc)
    }

    pub fn gt(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad > bc)
    }

    pub fn lt(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad < bc)
    }

    pub fn lte(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad <= bc)
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad == bc)
    }

    pub fn neq(&self, other: &Self) -> bool {
        self.compare_weights(other, |ad, bc| ad != bc)
    }

    pub fn checked_add(&self, other: &Self) -> Result<Self, WeightTableError> {
        let a = self.numerator();
        let b = self.denominator();
        let c = other.numerator();
        let d = other.denominator();

        // Calculate ad and bc
        let ad = a
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;
        let bc = b
            .checked_mul(c)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate numerator (ad + bc)
        let numerator = ad
            .checked_add(bc)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate denominator (bd)
        let denominator = b
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        let weight = Self {
            numerator: PodU64::from(numerator),
            denominator: PodU64::from(denominator),
        };

        weight.simplify()
    }

    pub fn checked_sub(&self, other: &Self) -> Result<Self, WeightTableError> {
        let a = self.numerator();
        let b = self.denominator();
        let c = other.numerator();
        let d = other.denominator();

        // Calculate ad and bc
        let ad = a
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;
        let bc = b
            .checked_mul(c)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate numerator (ad - bc)
        let numerator = ad
            .checked_sub(bc)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate denominator (bd)
        let denominator = b
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Check if the result is zero
        if numerator == 0 {
            return Ok(Self::default());
        }

        let weight = Self {
            numerator: PodU64::from(numerator),
            denominator: PodU64::from(denominator),
        };

        weight.simplify()
    }

    pub fn checked_mul(&self, other: &Self) -> Result<Self, WeightTableError> {
        let a = self.numerator();
        let b = self.denominator();
        let c = other.numerator();
        let d = other.denominator();

        // Calculate numerator (ac)
        let numerator = a
            .checked_mul(c)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate denominator (bd)
        let denominator = b
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        if denominator == 0 {
            return Err(WeightTableError::DenominatorIsZero);
        }

        let weight = Self {
            numerator: PodU64::from(numerator),
            denominator: PodU64::from(denominator),
        };

        weight.simplify()
    }

    pub fn checked_div(&self, other: &Self) -> Result<Self, WeightTableError> {
        if other.is_zero() {
            return Err(WeightTableError::DenominatorIsZero);
        }

        let a = self.numerator();
        let b = self.denominator();
        let c = other.numerator();
        let d = other.denominator();

        // Division by a/b / c/d is equivalent to multiplication by a/b * d/c
        // So we multiply by the reciprocal

        // Calculate numerator (ad)
        let numerator = a
            .checked_mul(d)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        // Calculate denominator (bc)
        let denominator = b
            .checked_mul(c)
            .ok_or(WeightTableError::ArithmeticOverflow)?;

        if denominator == 0 {
            return Err(WeightTableError::DenominatorIsZero);
        }

        let weight = Self {
            numerator: PodU64::from(numerator),
            denominator: PodU64::from(denominator),
        };

        weight.simplify()
    }
}

impl Default for Weight {
    fn default() -> Self {
        Self {
            numerator: PodU64::from(0),
            denominator: PodU64::from(1),
        }
    }
}

impl From<u64> for Weight {
    fn from(weight: u64) -> Self {
        Self {
            numerator: PodU64::from(weight),
            denominator: PodU64::from(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Weight::new(1, 2).is_ok());
        assert!(matches!(
            Weight::new(1, 0),
            Err(WeightTableError::DenominatorIsZero)
        ));
    }

    #[test]
    fn test_is_zero() {
        assert!(Weight::new(0, 1).unwrap().is_zero());
        assert!(!Weight::new(1, 2).unwrap().is_zero());
    }

    #[test]
    fn test_gcd() {
        assert_eq!(Weight::new(6, 8).unwrap().gcd().unwrap(), 2);
        assert_eq!(Weight::new(17, 23).unwrap().gcd().unwrap(), 1);

        let bad_weight = Weight::new(1, 0).unwrap_err();
        assert!(matches!(bad_weight, WeightTableError::DenominatorIsZero));
    }

    #[test]
    fn test_simplify() {
        let w1 = Weight::new(6, 8).unwrap().simplify().unwrap();
        assert_eq!(w1.numerator(), 3);
        assert_eq!(w1.denominator(), 4);

        let w2 = Weight::new(17, 23).unwrap().simplify().unwrap();
        assert_eq!(w2.numerator(), 17);
        assert_eq!(w2.denominator(), 23);
    }

    #[test]
    fn test_comparisons() {
        let w1 = Weight::new(1, 2).unwrap();
        let w2 = Weight::new(3, 4).unwrap();
        let w3 = Weight::new(1, 2).unwrap();

        assert!(w1.lt(&w2));
        assert!(w1.lte(&w2));
        assert!(w2.gt(&w1));
        assert!(w2.gte(&w1));
        assert!(w1.eq(&w3));
        assert!(w1.neq(&w2));
    }

    #[test]
    fn test_checked_add() {
        let w1 = Weight::new(1, 2).unwrap();
        let w2 = Weight::new(1, 4).unwrap();
        let result = w1.checked_add(&w2).unwrap();
        assert_eq!(result.numerator(), 3);
        assert_eq!(result.denominator(), 4);

        // Test overflow
        let w_max = Weight::new(u64::MAX, 1).unwrap();
        assert!(matches!(
            w_max.checked_add(&w1),
            Err(WeightTableError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn test_checked_sub() {
        let w1 = Weight::new(3, 4).unwrap();
        let w2 = Weight::new(1, 4).unwrap();
        let result = w1.checked_sub(&w2).unwrap();
        assert_eq!(result.numerator(), 1);
        assert_eq!(result.denominator(), 2);

        // Test underflow
        let w_min = Weight::new(0, 1).unwrap();
        assert!(matches!(
            w_min.checked_sub(&w1),
            Err(WeightTableError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn test_checked_mul() {
        let w1 = Weight::new(2, 3).unwrap();
        let w2 = Weight::new(3, 4).unwrap();
        let result = w1.checked_mul(&w2).unwrap();
        assert_eq!(result.numerator(), 1);
        assert_eq!(result.denominator(), 2);

        // Test overflow
        let w_max = Weight::new(u64::MAX, 1).unwrap();
        assert!(matches!(
            w_max.checked_mul(&w1),
            Err(WeightTableError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn test_checked_div() {
        let w1 = Weight::new(2, 3).unwrap();
        let w2 = Weight::new(3, 4).unwrap();
        let result = w1.checked_div(&w2).unwrap();
        assert_eq!(result.numerator(), 8);
        assert_eq!(result.denominator(), 9);

        // Test division by zero
        let w_zero = Weight::new(0, 1).unwrap();
        assert!(matches!(
            w1.checked_div(&w_zero),
            Err(WeightTableError::DenominatorIsZero)
        ));

        // Test overflow
        let w_max = Weight::new(u64::MAX, 1).unwrap();
        let w_min = Weight::new(1, u64::MAX).unwrap();
        assert!(matches!(
            w_max.checked_div(&w_min),
            Err(WeightTableError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn test_largest_and_smallest_comparison() {
        let largest = Weight::new(u64::MAX, 1).unwrap();
        let smallest = Weight::new(1, u64::MAX).unwrap();

        // Due to overflow protection, these comparisons will return false
        assert!(!largest.gt(&smallest));
        assert!(!smallest.lt(&largest));
        assert!(!largest.eq(&smallest));
    }

    #[test]
    fn test_large_number_comparison() {
        let large1 = Weight::new(u64::MAX / 2, 1).unwrap();
        let large2 = Weight::new(u64::MAX / 2 + 1, 1).unwrap();

        assert!(large2.gt(&large1));
        assert!(large1.lt(&large2));
        assert!(!large1.eq(&large2));
    }

    #[test]
    fn test_small_number_comparison() {
        let small1 = Weight::new(1, u64::MAX).unwrap();
        let small2 = Weight::new(2, u64::MAX).unwrap();

        // Due to precision limitations, these might not compare as expected
        assert!(!small2.gt(&small1));
        assert!(!small1.lt(&small2));
        assert!(!small1.eq(&small2));
    }

    #[test]
    fn test_precision_limit() {
        let w1 = Weight::new(u64::MAX / 2, u64::MAX / 2).unwrap();
        let w2 = Weight::new(u64::MAX / 2 + 1, u64::MAX / 2).unwrap();

        // Due to overflow protection, these comparisons will return false
        assert!(!w2.gt(&w1));
        assert!(!w1.lt(&w2));
        assert!(!w1.eq(&w2));
    }

    #[test]
    fn test_overflow_handling() {
        let w1 = Weight::new(u64::MAX, 2).unwrap();
        let w2 = Weight::new(u64::MAX - 1, 2).unwrap();

        // This comparison should return false due to overflow protection
        assert!(!w1.gt(&w2));
        assert!(!w1.lt(&w2));
        assert!(!w1.eq(&w2));
    }

    #[test]
    fn test_equality_of_simplified_weights() {
        let w1 = Weight::new(2, 4).unwrap();
        let w2 = Weight::new(1, 2).unwrap();

        assert!(w1.eq(&w2));
        assert!(!w1.gt(&w2));
        assert!(!w1.lt(&w2));
    }

    #[test]
    fn test_comparison_with_zero() {
        let zero = Weight::new(0, 1).unwrap();
        let smallest_positive = Weight::new(1, u64::MAX).unwrap();

        assert!(smallest_positive.gt(&zero));
        assert!(zero.lt(&smallest_positive));
        assert!(!zero.eq(&smallest_positive));
    }
}
