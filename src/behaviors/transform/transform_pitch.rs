use super::*;
use crate::types::pitch::{chord::*, scale::*, melody::*};

pub trait Transpose {
    fn transpose(&self, amount: i16) -> Self;
}

pub mod repeat {
    use super::*;
    use crate::types::pitch::melody::*;

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
}

pub mod rotate {
    use super::*;

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
}

pub mod transpose {
    use crate::types::pitch::melody::{MelodyClass, PitchCycle, PitchClassCycle};

    use super::*;

    impl Transpose for PitchSet {
        fn transpose(&self, amount: i16) -> Self {
            let numbers: Vec<i16> = self.numbers
                .iter()
                .map(|n| n + amount)
                .collect();

            Self::new(numbers)
        }
    }

    impl Transpose for PitchClassSet {
        fn transpose(&self, amount: i16) -> Self {
            let residue_classes: Vec<i16> = self.residue_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(residue_classes, self.modulus())
        }
    }

    impl Transpose for PitchScaleMap {
        fn transpose(&self, amount: i16) -> Self {
            Self::new(self.harmonics.clone(), self.offset + amount)
        }
    }

    impl Transpose for PitchScaleKey { 
        fn transpose(&self, amount: i16) -> Self {
            let residue_classes: Vec<i16> = self.residue_classes
                .iter()
                .map(|n| (n + amount).rem_euclid(self.modulus()))
                .collect();

            Self::new(residue_classes, self.modulus(), self.root() + amount)
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

