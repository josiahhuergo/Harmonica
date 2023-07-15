use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitch classes.
pub type PitchClassSet = ResidueSet<i16>;

/// A struct representing a patterned mapping of indices to pitches.
pub type PitchScaleMap = ScaleMap<i16>;

/// A struct representing an indexed `PitchClassSet`.
pub type PitchScaleKey = IndexedResidues<i16>;

/// A struct representing the "shape" of a pitch scale.
pub type PitchScaleShape = ScaleShape<i16>;

pub mod constructors { 
    use super::*;

    impl ConstructResidueSet<i16> for PitchClassSet {
        fn new(pitch_classes: Vec<i16>, modulus: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &pitch_class in pitch_classes.iter() {
                    assert!(pitch_class < modulus, "Pitch classes in PitchClassSet must be less than the modulus.");
                    assert!(collection_is_sorted(&pitch_classes), "Pitch classes in PitchClassSet must be in ascending order.");
                    assert!(pitch_class >= 0, "Pitch classes in PitchClassSet must be non-negative.");
                }
                assert!(modulus >= 0, "Modulus of PitchClassSet must be non-negative.");
            }
    
            Self { residue_classes: pitch_classes, modulus }
        }
    }

    impl ConstructScaleMap<i16> for PitchScaleMap {
        fn new(harmonics: Vec<i16>, transposition: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &harmonic in harmonics.iter() {
                    assert!(harmonic > 0, "Harmonics in PitchScaleMap must be positive.");
                }
                assert!(collection_is_unique(&harmonics), "Harmonics in PitchScaleMap must be unique.");
                assert!(collection_is_sorted(&harmonics), "Harmonics in PitchScaleMap must be in order.");
            }
    
            Self { harmonics, offset: transposition }
        }
    }

    impl ConstructIndexedResidues<i16> for PitchScaleKey {
        fn new(pitch_classes: Vec<i16>, modulus: i16, root: i16) -> Self {
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
    
            Self { residue_classes: pitch_classes, modulus }
        }
    }

    impl ConstructScaleShape<i16> for PitchScaleShape {
        fn new(intervals: Vec<i16>) -> Self {
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