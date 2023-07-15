use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitch classes.
/// 
/// ## Predicates
/// 
/// * Pitch classes must be in ascending order.
/// * Pitch classes & modulus must be non-negative.
/// * Pitch classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct PitchClassSet {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing a patterned mapping from indices to pitches.
/// 
/// ## Predicates
/// 
/// * Harmonics must be positive, unique, and in ascending order.
#[derive(PartialEq, Debug)]
pub struct PitchScaleMap {
    pub harmonics: Vec<i16>,
    pub transposition: i16
}

/// A struct representing an indexed pitch class set.
/// 
/// ## Predicates
/// 
/// * Pitch classes and modulus must be non-negative.
/// * Pitch classes must be unique. 
/// * Pitch classes must be less than the modulus. 
/// * Pitch classes must be in cyclically ascending order.
#[derive(PartialEq, Debug)]
pub struct PitchScaleKey {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing the shape of a scale.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct PitchScaleShape {
    pub intervals: Vec<i16>
}

pub mod constructors { 
    use super::*;

    impl PitchClassSet {
        pub fn new(pitch_classes: Vec<i16>, modulus: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &pitch_class in pitch_classes.iter() {
                    assert!(pitch_class < modulus, "Pitch classes in PitchClassSet must be less than the modulus.");
                    assert!(collection_is_sorted(&pitch_classes), "Pitch classes in PitchClassSet must be in ascending order.");
                    assert!(pitch_class >= 0, "Pitch classes in PitchClassSet must be non-negative.");
                }
                assert!(modulus >= 0, "Modulus of PitchClassSet must be non-negative.");
            }
    
            Self { pitch_classes, modulus }
        }
    }

    impl PitchScaleMap {
        pub fn new(harmonics: Vec<i16>, transposition: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &harmonic in harmonics.iter() {
                    assert!(harmonic > 0, "Harmonics in PitchScaleMap must be positive.");
                }
                assert!(collection_is_unique(&harmonics), "Harmonics in PitchScaleMap must be unique.");
                assert!(collection_is_sorted(&harmonics), "Harmonics in PitchScaleMap must be in order.");
            }
    
            Self { harmonics, transposition }
        }
    }

    impl PitchScaleKey {
        pub fn new(pitch_classes: Vec<i16>, modulus: i16, root: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &pitch_class in pitch_classes.iter() {
                    assert!(pitch_class < modulus, "Pitch classes in ScaleKey must be less than the modulus.");
                    assert!(pitch_class >= 0, "Pitch classes in ScaleKey must be non-negative.");
                }
                assert!(pitch_classes.contains(&root), "Pitch classes in ScaleKey must contain root.");
                assert!(modulus >= 0, "Modulus of ScaleKey must be non-negative.");
            }
    
            let pitch_classes = cyclically_order_vector(&pitch_classes, root);
    
            Self { pitch_classes, modulus }
        }
    }

    impl PitchScaleShape {
        pub fn new(intervals: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0, "Intervals in ScaleShape must be positive.");
                }
            }
            Self { intervals }
        }
    }
}