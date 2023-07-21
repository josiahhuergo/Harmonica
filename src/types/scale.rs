use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitch classes.
/// 
/// ## Predicates
/// 
/// * Pitch classes must be in ascending order.
/// * Pitch classes must be unique.
/// * Pitch classes & modulus must be non-negative.
/// * Pitch classes must be less than the modulus.
#[derive(PartialEq, Debug, Clone)]
pub struct Scale {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing a patterned mapping from indices to pitches.
/// 
/// ## Predicates
/// 
/// * Harmonics must be positive.
/// * Harmonics must be unique.
/// * Harmonics must be in ascending order.
#[derive(PartialEq, Debug)]
pub struct ScaleMap {
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
#[derive(PartialEq, Debug, Clone)]
pub struct ScaleKey {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing the shape of a scale.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct ScaleShape {
    pub intervals: Vec<i16>
}

pub mod constructors { 
    use super::*;

    impl Scale {
        pub fn new(pitch_classes: Vec<i16>, modulus: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &pitch_class in pitch_classes.iter() {
                    assert!(pitch_class < modulus, "Pitch classes in Scale must be less than the modulus.");
                    assert!(pitch_class >= 0, "Pitch classes in Scale must be non-negative.");
                }
                assert!(collection_is_sorted(&pitch_classes), "Pitch classes in Scale must be in ascending order.");
                assert!(modulus >= 0, "Modulus of Scale must be non-negative.");
            }
    
            Self { pitch_classes, modulus }
        }
    }

    impl ScaleMap {
        pub fn new(harmonics: Vec<i16>, transposition: i16) -> Self {
            #[cfg(debug_assertions)]
            {
                for &harmonic in harmonics.iter() {
                    assert!(harmonic > 0, "Harmonics in ScaleMap must be positive.");
                }
                assert!(collection_is_unique(&harmonics), "Harmonics in ScaleMap must be unique.");
                assert!(collection_is_sorted(&harmonics), "Harmonics in ScaleMap must be in order.");
            }
    
            Self { harmonics, transposition }
        }
    }

    impl ScaleKey {
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

    impl ScaleShape {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_scale() {
        let scale = Scale::new(vec![-2, 2, 3], 3);
    }

    #[test]
    #[should_panic]
    fn test_scale_map() {
        let scale_map = ScaleMap::new(vec![-2, 3, 2], 4);
    }

    #[test]
    #[should_panic]
    fn test_scale_key() {
        let scale_key = ScaleKey::new(vec![1,3,4,6], 7, 5);
    }

    #[test]
    #[should_panic]
    fn test_pitch_scale_shape() {
        let pitch_scale_shape = ScaleShape::new(vec![-1,2,6,2,3]);
    }
}