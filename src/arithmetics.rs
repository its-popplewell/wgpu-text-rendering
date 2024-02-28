use num::{One, Zero};
use std::ops::{Mul, Sub, Add};

pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// Returns the middle of the three values
pub fn median<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    max(min(a, b), min(max(a, b), c))
}

/// Returns weighted avg of a and b
pub fn mix<T, S>(a: T, b: T, w: S) -> T 
where 
    T: Mul<S, Output = T> + Add<Output = T> + Copy,
    S: Sub<Output = S> + Mul<T, Output = T> + One + Copy,
{
    ((S::one() - w) * a) + (w * b)
}

/// Clamp between zero and one
pub fn clamp<T: PartialOrd + One + Zero + Copy>(x: T) -> T {
    if x > T::one() {
        T::one()
    } else if x < T::zero() {
        T::zero()
    } else {
        x
    }
}

pub fn clamp_to_ray<T: PartialOrd + Zero + Copy>(x: T, lim: T) -> T {

    assert!(lim > T::zero());

    if x < T::zero() {
        T::zero()
    } else if x > lim {
        lim
    } else {
        x
    }
}

/// Clamps between min and max
pub fn clamp_between<T: PartialOrd + Copy>(x: T, min: T, max: T) -> T {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}

/// Returns 1 for pos, -1 for neg, 0 for zero
pub fn sign<T: PartialOrd + Zero + Copy>(x: T) -> i8 {
    if x > T::zero() {
        1
    } else if x < T::zero() {
        -1
    } else {
        0
    }
}

// returns 1 for non negative values, -1 for negative values
pub fn non_zero_sign<T: PartialOrd + Zero + Copy> (x: T) -> i8 {
    if x >= T::zero() {
        1
    } else {
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(min(1, 2), 1);
        assert_eq!(min(2, 1), 1);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(1, 2), 2);
        assert_eq!(max(2, 1), 2);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(1, 2, 3), 2);
        assert_eq!(median(3, 2, 1), 2);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(1., 2., 0.5), 1.5);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(1.5), 1.0);
        assert_eq!(clamp(-1.5), 0.0);
    }

    #[test]
    fn test_clamp_to_ray() {
        assert_eq!(clamp_to_ray(1.5, 2.0), 1.5);
        assert_eq!(clamp_to_ray(-1.5, 2.0), 0.0);
    }

    #[test]
    fn test_clamp_between() {
        assert_eq!(clamp_between(1.5, 1.0, 2.0), 1.5);
        assert_eq!(clamp_between(0.5, 1.0, 2.0), 1.0);
        assert_eq!(clamp_between(2.5, 1.0, 2.0), 2.0);
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(1), 1);
        assert_eq!(sign(-1), -1);
        assert_eq!(sign(0), 0);
    }

    #[test]
    fn test_non_zero_sign() {
        assert_eq!(non_zero_sign(1), 1);
        assert_eq!(non_zero_sign(-1), -1);
        assert_eq!(non_zero_sign(0), 1);
    }

}
