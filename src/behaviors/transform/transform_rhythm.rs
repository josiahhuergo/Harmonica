use super::*;
use crate::types::rhythm::*;

/// A trait representing the offset of times in a rhythmic struct.
pub trait Offset {
    /// Offsets the times in a struct by amount.
    fn offset(&self, amount: f64) -> Self;
}

pub mod offset {
    use super::*;

    impl Offset for TimeSet {
        fn offset(&self, amount: f64) -> Self {
            let numbers: Vec<f64> = self.numbers
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(numbers)
        }
    }

    impl Offset for TimeClassSet {
        fn offset(&self, amount: f64) -> Self {
            let time_classes: Vec<f64> = self.residue_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(time_classes, self.modulus())
        }
    }

    impl Offset for TimeScaleMap {
        fn offset(&self, amount: f64) -> Self {
            Self::new(self.harmonics.clone(), self.offset + amount)
        }
    }

    impl Offset for TimeScaleKey { 
        fn offset(&self, amount: f64) -> Self {
            let time_classes: Vec<f64> = self.residue_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(time_classes, self.modulus(), self.root() + amount)
        }
    }
}

