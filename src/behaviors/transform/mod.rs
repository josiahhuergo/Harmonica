use crate::types::*;
use crate::utility::*;
use crate::behaviors::analyze::*;
use crate::types::{pitch::scale::*, rhythm::*};
use std::ops::{Sub, Rem, Add};

/// Transform Pitch Module
/// 
/// The `transform_pitch` module provides behaviors related to the transformation of pitch types.
pub mod transform_pitch;

/// Transform Rhythm Module
/// 
/// The `transform_rhythm` module provides behaviors related to the transformation of rhythm types.
pub mod transform_rhythm;

/// A trait representing the rotation of a scale's modes.
pub trait RotateMode {
    /// Rotates the mode of the scale in a parallel way.
    fn parallel_rotate(&self, amount: i16) -> Self;

    /// Rotates the mode of the scale in a relative way.
    fn relative_rotate(&self, amount: i16) -> Self;
}

/// A trait representing the repetition of elements in a sequence,
pub trait Repeat {
    /// Repeats a sequence n times.
    fn repeat(&self, n: usize) -> Self;

    /// Repeats each element of a sequence n times.
    fn stretch(&self, n: usize) -> Self;
}

/// A trait representing the rotation of a cyclical collection of things.
pub trait Rotate {
    /// Rotates a cycle n times.
    fn rotate(&self, n: i16) -> Self;
}

pub mod rotate_mode {
    use super::*;

    impl<T> RotateMode for ScaleMap<T> 
    where
        T: Copy + Default + PartialOrd + PartialEq + Rem<Output = T> + Add<Output = T> + Sub<Output = T>,
        ScaleMap<T>: Modulus<T> + Eval<T> + ConstructScaleMap<T>
    {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let m = self.modulus();

            let harmonics: Vec<T> = self.harmonics.iter().cloned()
                .map(|num| (((num - self.eval(amount)) % m) + m) % m)
                .filter(|num| *num != T::default())
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(harmonics, self.offset)
        }

        fn relative_rotate(&self, amount: i16) -> Self {
            let len = self.len() as i16;
            let m = self.modulus();

            let sub: T = self.harmonics[(((amount % len) + len) % len) as usize - 1];

            let harmonics: Vec<T> = self.harmonics.iter().cloned()
                .map(|num| (((num - sub) % m) + m) % m)
                .filter(|num| *num != T::default())
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(sort_vector(&harmonics), self.eval(amount))
        }
    }

    impl RotateMode for TimeScaleKey {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let m = self.len() as i16;
            let t = self.residue_classes[amount.rem_euclid(m) as usize] - self.root();
            let residue_classes: Vec<f64> = self.residue_classes
                .iter()
                .map(|num| (*num - t).rem_euclid(m as f64))
                .collect();
    
            Self::new(residue_classes, self.modulus(), self.root())
        }
    
        fn relative_rotate(&self, amount: i16) -> Self {
            Self::new(self.residue_classes.clone(), self.modulus(), self.eval(amount.rem_euclid(self.len() as i16)))
        }
    }

    impl RotateMode for PitchScaleKey
    {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let m = self.len() as i16;
            let t = self.residue_classes[amount.rem_euclid(m) as usize] - self.root();
            let residue_classes: Vec<i16> = self.residue_classes
                .iter()
                .map(|num| (*num - t).rem_euclid(m))
                .collect();
    
            Self::new(residue_classes, self.modulus(), self.root())
        }
    
        fn relative_rotate(&self, amount: i16) -> Self {
            Self::new(self.residue_classes.clone(), self.modulus(), self.eval(amount.rem_euclid(self.len() as i16)))
        }
    }
}

pub mod repeat {
    use super::*;

    impl<T: Clone> Repeat for Shape<T> 
    where Shape<T>: ConstructShape<T> {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }
    
        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl<T: Clone> Repeat for ScaleShape<T>
    where ScaleShape<T>: ConstructScaleShape<T> {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }
}

pub mod rotate {
    use super::*;

    impl<T> Rotate for Shape<T> 
    where
        T: Clone,
        Shape<T>: ConstructShape<T> + Len
    {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }

    impl<T> Rotate for ScaleShape<T> 
    where
        T: Clone,
        ScaleShape<T>: ConstructScaleShape<T> + Len
    {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }
}

