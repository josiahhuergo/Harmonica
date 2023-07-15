/// A struct representing a sequence of pitches.
#[derive(PartialEq, Debug)]
pub struct Melody {
    pub pitches: Vec<i16>,
}

/// A struct representing the "shape" of a sequence of pitches, i.e. the differences between adjacent pitches.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct MelodyShape {
    pub intervals: Vec<i16>
}

/// A struct representing a sequence of pitch classes.
/// 
/// ## Predicates
/// 
/// * Pitch classes & modulus must be non-negative.
/// * Pitch classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct MelodyClass {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16,
}

/// A struct representing the "shape" of a sequence of pitch classes.
/// 
/// ## Predicates
/// 
/// * Interval classes & modulus must be non-negative.
/// * Interval classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct MelodyClassShape {
    pub interval_classes: Vec<i16>,
    pub modulus: i16,
}

pub type PitchCycle = Melody;
pub type IntervalCycle = MelodyShape;
pub type PitchClassCycle = MelodyClass;
pub type IntervalClassCycle = MelodyClassShape;

pub mod constructors {
    use super::*;

    impl Melody {
        pub fn new(pitches: Vec<i16>) -> Self {
            Self { pitches }
        }
    }

    impl MelodyShape {
        pub fn new(intervals: Vec<i16>) -> Self {
            Self { intervals }
        }
    }

    impl MelodyClass {
        pub fn new(pitch_classes: Vec<i16>, modulus: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &pitch_class in pitch_classes.iter() {
                    assert!(pitch_class < modulus, "Pitch classes in MelodyClass must be less than the modulus.");
                    assert!(pitch_class >= 0, "Pitch classes in MelodyClass must be non-negative.");
                }
                assert!(modulus >= 0, "Modulus of MelodyClass must be non-negative.");
            }

            Self { pitch_classes, modulus }
        }
    }

    impl MelodyClassShape {
        pub fn new(interval_classes: Vec<i16>, modulus: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval_class in interval_classes.iter() {
                    assert!(interval_class < modulus, "Interval classes in MelodyClassShape must be less than the modulus.");
                    assert!(interval_class >= 0, "Interval classes in MelodyClassShape must be non-negative.");
                }
                assert!(modulus >= 0, "Modulus of MelodyClassShape must be non-negative.");
            }

            Self { interval_classes, modulus }
        }
    }
}