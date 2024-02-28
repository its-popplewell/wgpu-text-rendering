struct SignedDistance {
    pub distance: f64,
    pub dot: f64,
}

impl Default for SignedDistance {
    fn default() -> Self {
        Self {
            distance: -f64::MAX,
            dot: 0.,
        }
    }
}

impl SignedDistance {
    pub fn new(distance: f64, dot: f64) -> Self {
        Self { distance, dot }
    }
}

impl PartialOrd for SignedDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for SignedDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_distance() {
        let a = SignedDistance::new(1., 0.);
        let b = SignedDistance::new(2., 0.);
        assert!(a < b);
        assert!(b > a);
        assert!(b >= a);
        assert!(a <= b);
    }


}
