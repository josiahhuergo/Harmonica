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

    impl Rotate for ChordShape {
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

    impl RotateMode for PitchScaleKey {
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

    impl RotateMode for MelodicMap {
        fn parallel_rotate(&self, amount: i16) -> Self {
            let t = self.eval(amount);

            let mut harmonics = Vec::<i16>::new();

            for i in (amount + 1)..(self.len() as i16 + amount + 1) {
                harmonics.push(self.eval(i) - t);
            }

            Self::new(harmonics, self.transposition)
        }

        fn relative_rotate(&self, amount: i16) -> Self {
            let t = self.eval(amount);

            let mut harmonics = Vec::<i16>::new();

            for i in (amount + 1)..(self.len() as i16 + amount + 1) {
                harmonics.push(self.eval(i) - t);
            }

            Self::new(harmonics, t)
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

    impl Repeat for ChordShape {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }
    
        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for PitchClassSet {
        fn repeat(&self, n: usize) -> Self {
            self.shape().repeat(n).stamp(*self.pitch_classes.first().unwrap())
        }

        fn stretch(&self, n: usize) -> Self {
            self.shape().stretch(n).stamp(*self.pitch_classes.first().unwrap())
        }
    }

    impl Repeat for PitchScaleMap {
        fn repeat(&self, n: usize) -> Self {
            self.shape().repeat(n).stamp_to_scale_map(self.transposition)
        }

        fn stretch(&self, n: usize) -> Self {
            self.shape().stretch(n).stamp_to_scale_map(self.transposition)
        }
    }

    impl Repeat for PitchScaleKey {
        fn repeat(&self, n: usize) -> Self {
            self.shape().repeat(n).stamp_to_scale_key(self.root())
        }

        fn stretch(&self, n: usize) -> Self {
            self.shape().stretch(n).stamp_to_scale_key(self.root())
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

    impl Repeat for MelodicMap {
        fn repeat(&self, n: usize) -> Self {
            self.shape().repeat(n).stamp(self.transposition)
        }

        fn stretch(&self, n: usize) -> Self {
            self.shape().stretch(n).stamp(self.transposition)
        }
    }

    impl Repeat for PitchCycle {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.pitches, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.pitches, n))
        }
    }

    impl Repeat for IntervalCycle {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.intervals, n))
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.intervals, n))
        }
    }

    impl Repeat for PitchClassCycle {
        fn repeat(&self, n: usize) -> Self {
            Self::new(repeat_list(&self.pitch_classes, n), self.modulus)
        }

        fn stretch(&self, n: usize) -> Self {
            Self::new(stretch_list(&self.pitch_classes, n), self.modulus)
        }
    }

    impl Repeat for IntervalClassCycle {
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

    impl Transpose for Chord {
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

    impl Transpose for MelodicMap {
        fn transpose(&self, amount: i16) -> Self {
            Self::new(self.harmonics.clone(), self.transposition + amount)
        }
    }

    impl Transpose for PitchCycle {
        fn transpose(&self, amount: i16) -> Self {
            let numbers: Vec<i16> = self.pitches
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(numbers)
        }
    }

    impl Transpose for PitchClassCycle {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod rotate {
        use super::*;

        #[test]
        fn test_chord_shape() {
            let chord_shape = ChordShape::new(vec![4,6,3,2]);
            let rotation = chord_shape.rotate(2);
            let result = ChordShape::new(vec![3,2,4,6]);

            assert_eq!(rotation, result);
        }

        #[test]
        fn test_pitch_scale_shape() {
            let pitch_scale_shape = PitchScaleShape::new(vec![4,7,2,4]);
            let rotation = pitch_scale_shape.rotate(2);
            let result = PitchScaleShape::new(vec![2,4,4,7]);

            assert_eq!(rotation, result);
        }

        #[test]
        fn test_pitch_cycle() {
            let pitch_cycle = PitchCycle::new(vec![2,7,3,-3]);
            let rotation = pitch_cycle.rotate(2);
            let result = PitchCycle::new(vec![3,-3,2,7]);

            assert_eq!(rotation, result);
        }

        #[test]
        fn test_interval_cycle() {
            let interval_cycle = IntervalCycle::new(vec![2,7,3,-3]);
            let rotation = interval_cycle.rotate(2);
            let result = IntervalCycle::new(vec![3,-3,2,7]);

            assert_eq!(rotation, result);
        }

        #[test]
        fn test_pitch_class_cycle() {
            let pitch_class_cycle = PitchClassCycle::new(vec![2,7,3,3], 12);
            let rotation = pitch_class_cycle.rotate(2);
            let result = PitchClassCycle::new(vec![3,3,2,7], 12);

            assert_eq!(rotation, result);
        }

        #[test]
        fn test_interval_class_cycle() {
            let interval_class_cycle = IntervalClassCycle::new(vec![2,7,3,3], 12);
            let rotation = interval_class_cycle.rotate(2);
            let result = IntervalClassCycle::new(vec![3,3,2,7], 12);

            assert_eq!(rotation, result);
        }
    }

    mod rotate_mode {
        use super::*;

        mod parallel {
            use super::*;

            #[test]
            fn test_pitch_scale_map() {
                let pitch_scale_map = PitchScaleMap::new(vec![2,3,5,7], 2);
                let rotation = pitch_scale_map.parallel_rotate(2);
                let result = PitchScaleMap::new(vec![2,4,6,7], 2);

                assert_eq!(rotation, result);
            }

            #[test]
            fn test_pitch_scale_key() {
                let pitch_scale_key = PitchScaleKey::new(vec![0,2,3,5], 7, 0);
                let rotation = pitch_scale_key.parallel_rotate(2);
                let result = PitchScaleKey::new(vec![0,2,4,6], 7, 0);

                assert_eq!(rotation, result);
            }

            #[test]
            fn test_melodic_map() {
                let melodic_map = MelodicMap::new(vec![2,-1,5], 0);
                let rotation = MelodicMap::new(vec![6,8,5], 0);

                assert_eq!(melodic_map.parallel_rotate(2), rotation);
            }
        }

        mod relative {
            use super::*;

            #[test]
            fn test_pitch_scale_map() {
                let pitch_scale_map = PitchScaleMap::new(vec![2,3,5,7], 2);
                let rotation = pitch_scale_map.relative_rotate(2);
                let result = PitchScaleMap::new(vec![2,4,6,7], 5);

                assert_eq!(rotation, result);
            }

            #[test]
            fn test_pitch_scale_key() {
                let pitch_scale_key = PitchScaleKey::new(vec![0,2,3,5], 7, 0);
                let rotation = pitch_scale_key.relative_rotate(2);
                let result = PitchScaleKey::new(vec![0,2,3,5], 7, 3);

                assert_eq!(rotation, result);
            }

            #[test]
            fn test_melodic_map() {
                let melodic_map = MelodicMap::new(vec![2,-1,5], 0);
                let rotation = MelodicMap::new(vec![6,8,5], -1);

                assert_eq!(melodic_map.relative_rotate(2), rotation);
            }
        }
    }

    mod repeat {
        use super::*;

        #[test]
        fn test_repeat_melody() {
            let melody = Melody::new(vec![2,7,3,4,8]);
            let repeat = melody.repeat(2);
            let result = Melody::new(vec![2,7,3,4,8,2,7,3,4,8]);

            assert_eq!(repeat, result);
        }

        #[test]
        fn test_stretch_melody() {
            let melody = Melody::new(vec![2,7,3,4,8]);
            let stretch = melody.stretch(2);
            let result = Melody::new(vec![2,2,7,7,3,3,4,4,8,8]);

            assert_eq!(stretch, result);
        }
    }

    mod transpose {
        use super::*;

        #[test]
        fn test_chord() {
            let chord = Chord::new(vec![0,3,6,12]);
            let transposition = Chord::new(vec![-4,-1,2,8]);

            assert_eq!(chord.transpose(-4), transposition);
        }

        #[test]
        fn test_pitch_class_set() {
            let pitch_class_set = PitchClassSet::new(vec![0,2,3,6,9], 12);
            let transposition = PitchClassSet::new(vec![0,1,4,7,10], 12);

            assert_eq!(pitch_class_set.transpose(-2), transposition);
        }

        #[test]
        fn test_pitch_scale_map() {
            let pitch_scale_map = PitchScaleMap::new(vec![2,3,5,7], 3);
            let transposition = PitchScaleMap::new(vec![2,3,5,7], 11);

            assert_eq!(pitch_scale_map.transpose(8), transposition);
        }

        #[test]
        fn test_pitch_scale_key() {
            let pitch_scale_key = PitchScaleKey::new(vec![0,2,3,5], 7, 3);
            let transposition = PitchScaleKey::new(vec![0,1,3,5], 7, 1);

            assert_eq!(pitch_scale_key.transpose(-2), transposition);
        }

        #[test]
        fn test_melodic_map() {
            let melodic_map = MelodicMap::new(vec![4,-2,6,1], 3);
            let transposition = MelodicMap::new(vec![4,-2,6,1], -1);

            assert_eq!(melodic_map.transpose(-4), transposition);
        }
    }
}