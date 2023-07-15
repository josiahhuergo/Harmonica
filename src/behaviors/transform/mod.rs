use crate::types::*;
use crate::utility::*;
use crate::behaviors::analyze::*;
use crate::types::{pitch::{scale::*, chord::*, melody::*}, rhythm::*};
use std::ops::{Sub, Rem, Add};

/// A trait representing the rotation of a cyclical collection of things.
pub trait Rotate {
    /// Rotates a cycle n times.
    fn rotate(&self, n: i16) -> Self;
}

/// A trait representing the rotation of a scale's modes.
pub trait RotateMode {
    /// Rotates the mode of the scale in a parallel way.
    fn parallel_rotate(&self, amount: i16) -> Self;

    /// Rotates the mode of the scale in a relative way.
    fn relative_rotate(&self, amount: i16) -> Self;
}

/// A trait representing the repetition of elements in a sequence.
pub trait Repeat {
    /// Repeats a sequence n times.
    fn repeat(&self, n: usize) -> Self;

    /// Repeats each element of a sequence n times.
    fn stretch(&self, n: usize) -> Self;
}

/// A trait representing the transposition of pitches in a pitch struct.
pub trait Transpose {
    /// Transposes the pitches in a struct by amount.
    fn transpose(&self, amount: i16) -> Self;
}

/// A trait representing the offset of times in a rhythmic struct.
pub trait Offset {
    /// Offsets the times in a struct by amount.
    fn offset(&self, amount: f64) -> Self;
}

//--------------------------------------------------------------------//

pub mod rotate {
    use super::*;

    impl Rotate for PitchSetShape {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }

    impl Rotate for PitchScaleShape {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }

    impl Rotate for PitchCycle {
        fn rotate(&self, n: i16) -> Self {
            let mut pitches = self.pitches.clone();
            pitches.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(pitches)
        }
    }

    impl Rotate for IntervalCycle {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }

    impl Rotate for PitchClassCycle {
        fn rotate(&self, n: i16) -> Self {
            let mut pitch_classes = self.pitch_classes.clone();
            pitch_classes.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(pitch_classes, self.modulus())
        }
    }

    impl Rotate for IntervalClassCycle {
        fn rotate(&self, n: i16) -> Self {
            let mut interval_classes = self.interval_classes.clone();
            interval_classes.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(interval_classes, self.modulus())
        }
    }

    impl Rotate for TimeSetShape {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }

    impl Rotate for TimeScaleShape {
        fn rotate(&self, n: i16) -> Self {
            let mut intervals = self.intervals.clone();
            intervals.rotate_left(n.rem_euclid(self.len() as i16) as usize);

            Self::new(intervals)
        }
    }
}

pub mod rotate_mode {
    use super::*;

    impl RotateMode for PitchScaleMap {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let harmonics = self.harmonics.iter().cloned()
                .map(|num| (num - self.eval(amount)).rem_euclid(self.modulus()))
                .filter(|num| *num != 0)
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(harmonics, self.transposition)
        }

        fn relative_rotate(&self, amount: i16) -> Self {
            let sub = self.harmonics[amount.rem_euclid(self.len() as i16) as usize - 1];

            let harmonics: Vec<i16> = self.harmonics.iter().cloned()
                .map(|num| (num - sub).rem_euclid(self.modulus()))
                .filter(|num| *num != 0)
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(sort_vector(&harmonics), self.eval(amount))
        }
    }

    impl RotateMode for PitchScaleKey
    {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let t = self.pitch_classes[amount.rem_euclid(self.len() as i16) as usize] - self.root();

            let pitch_classes: Vec<i16> = self.pitch_classes
                .iter()
                .map(|num| (*num - t).rem_euclid(self.modulus()))
                .collect();
    
            Self::new(pitch_classes, self.modulus(), self.root())
        }
    
        fn relative_rotate(&self, amount: i16) -> Self {
            Self::new(self.pitch_classes.clone(), self.modulus(), self.eval(amount.rem_euclid(self.len() as i16)))
        }
    }

    impl RotateMode for TimeScaleMap {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let harmonics = self.harmonics.iter().cloned()
                .map(|num| (num - self.eval(amount)).rem_euclid(self.modulus()))
                .filter(|num| *num != 0.0)
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(harmonics, self.offset)
        }

        fn relative_rotate(&self, amount: i16) -> Self {
            let sub = self.harmonics[amount.rem_euclid(self.len() as i16) as usize - 1];

            let harmonics: Vec<f64> = self.harmonics.iter().cloned()
                .map(|num| (num - sub).rem_euclid(self.modulus()))
                .filter(|num| *num != 0.0)
                .chain(vec![self.modulus()].into_iter()) 
                .collect();

            Self::new(sort_vector(&harmonics), self.eval(amount))
        }
    }

    impl RotateMode for TimeScaleKey {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let t = self.time_classes[amount.rem_euclid(self.len() as i16) as usize] - self.root();

            let time_classes: Vec<f64> = self.time_classes
                .iter()
                .map(|num| (*num - t).rem_euclid(self.modulus() as f64))
                .collect();
    
            Self::new(time_classes, self.modulus(), self.root())
        }
    
        fn relative_rotate(&self, amount: i16) -> Self {
            Self::new(self.time_classes.clone(), self.modulus(), self.eval(amount.rem_euclid(self.len() as i16)))
        }
    }
}

pub mod repeat {
    use super::*;

    impl Repeat for PitchSetShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }
    
        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for PitchScaleShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for Melody {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.pitches, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.pitches, n))
        }
    }

    impl Repeat for MelodyShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for MelodyClass {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.pitch_classes, n), self.modulus)
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.pitch_classes, n), self.modulus)
        }
    }

    impl Repeat for MelodyClassShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.interval_classes, n), self.modulus)
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.interval_classes, n), self.modulus)
        }
    }

    impl Repeat for TimeSetShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }
    
        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for TimeScaleShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }
}

pub mod transpose {
    use super::*;

    impl Transpose for PitchSet {
        fn transpose(&self, amount: i16) -> Self {
            let pitches: Vec<i16> = self.pitches
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(pitches)
        }
    }

    impl Transpose for PitchClassSet {
        fn transpose(&self, amount: i16) -> Self {
            let pitch_classes: Vec<i16> = self.pitch_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(pitch_classes, self.modulus())
        }
    }

    impl Transpose for PitchScaleMap {
        fn transpose(&self, amount: i16) -> Self {
            Self::new(self.harmonics.clone(), self.transposition + amount)
        }
    }

    impl Transpose for PitchScaleKey { 
        fn transpose(&self, amount: i16) -> Self {
            let pitch_classes: Vec<i16> = self.pitch_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(pitch_classes, self.modulus(), self.root() + amount)
        }    
    }

    impl Transpose for Melody {
        fn transpose(&self, amount: i16) -> Self {
            let numbers: Vec<i16> = self.pitches
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(numbers)
        }
    }

    impl Transpose for MelodyClass {
        fn transpose(&self, amount: i16) -> Self {
            let residue_classes: Vec<i16> = self.pitch_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(residue_classes, self.modulus())
        }
    }
}

pub mod offset {
    use super::*;

    impl Offset for TimeSet {
        fn offset(&self, amount: f64) -> Self {
            let times: Vec<f64> = self.times
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(times)
        }
    }

    impl Offset for TimeClassSet {
        fn offset(&self, amount: f64) -> Self {
            let time_classes: Vec<f64> = self.time_classes
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
            let time_classes: Vec<f64> = self.time_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(time_classes, self.modulus(), self.root() + amount)
        }
    }
}