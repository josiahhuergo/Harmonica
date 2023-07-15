use crate::types::*;
use crate::utility::*;

//-----------------------------------------------
//-------------------- SCALE --------------------
//-----------------------------------------------

/// A struct representing a set of time classes.
pub type TimeClassSet = ResidueSet<f64>;

/// A struct representing a patterned mapping from indexes to times.
pub type TimeScaleMap = ScaleMap<f64>;

/// A struct representing an indexed `TimeClassSet`.
pub type TimeScaleKey = IndexedResidues<f64>;

/// A struct representing the "shape" of a time scale.
pub type TimeScaleShape = ScaleShape<f64>;

//-----------------------------------------------
//--------------------- SET ---------------------
//-----------------------------------------------

/// A struct representing a set of times.
pub type TimeSet = Set<f64>;

/// A struct representing the shape of a `TimeSet`.
pub type TimeShape = Shape<f64>;

pub mod constructors {
    use super::*;

    impl ConstructResidueSet<f64> for TimeClassSet {
        fn new(time_classes: Vec<f64>, modulus: f64) -> Self {
            #[cfg(debug_assertions)]
            {
                for &time_class in time_classes.iter() {
                    assert!(time_class < modulus, "Time classes in TimeClassSet must be less than the modulus.");
                    assert!(floats_are_sorted(&time_classes), "Time classes in TimeClassSet must be in ascending order.");
                    assert!(time_class >= 0.0, "Time classes in TimeClassSet must be non-negative.");
                }
                assert!(modulus >= 0.0, "Modulus of TimeClassSet must be non-negative.");
            }
    
            Self { residue_classes: time_classes, modulus }
        }
    }

    impl ConstructScaleMap<f64> for TimeScaleMap {
        fn new(harmonics: Vec<f64>, offset: f64) -> Self {
            #[cfg(debug_assertions)]
            {
                for &harmonic in harmonics.iter() {
                    assert!(harmonic > 0.0, "Harmonics in TimeScaleMap must be positive.");
                }
                assert!(floats_are_unique(&harmonics), "Harmonics in TimeScaleMap must be unique.");
                assert!(floats_are_sorted(&harmonics), "Harmonics in TimeScaleMap must be in order.");
            }
    
            Self { harmonics, offset }
        }
    }

    impl ConstructIndexedResidues<f64> for TimeScaleKey {
        fn new(time_classes: Vec<f64>, modulus: f64, root: f64) -> Self {
            #[cfg(debug_assertions)]
            {
                for &time_class in time_classes.iter() {
                    assert!(time_class < modulus, "Time classes in TimeScaleKey must be less than the modulus.");
                    assert!(time_class >= 0.0, "Time classes in TimeScaleKey must be non-negative.");
                }
                assert!(time_classes.contains(&root), "Time classes in TimeScaleKey must contain root.");
                assert!(modulus >= 0.0, "Modulus of TimeScaleKey must be non-negative.");
            }
    
            let time_classes = cyclically_order_floats(&time_classes, root);
    
            Self { residue_classes: time_classes, modulus }
        }
    }

    impl ConstructScaleShape<f64> for TimeScaleShape {
        fn new(intervals: Vec<f64>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0.0, "Intervals in TimeScaleShape must be positive.");
                }
            }
            Self { intervals }
        }
    }

    impl ConstructSet<f64> for TimeSet {
        fn new(times: Vec<f64>) -> Self {
            let mut times = times.clone();
            times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            times.dedup();
    
            Self { numbers: times }
        }
    }

    impl ConstructShape<f64> for TimeShape {
        fn new(intervals: Vec<f64>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0.0, "Intervals must be positive.");
                }
            }
    
            Self { intervals }
        }
    }
}