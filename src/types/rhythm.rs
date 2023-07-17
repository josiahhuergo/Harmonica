use crate::types::*;
use crate::utility::*;

/* Rhythm types and behaviors are on hold
   until I find a suitable replacement
   for using floats, because floats suck. */

//-----------------------------------------------
//--------------------- SET ---------------------
//-----------------------------------------------

/// A struct representing a set of times.
/// 
/// ## Predicates
/// 
/// * Times must be unique.
/// * Times must be in ascending order.
#[derive(PartialEq, Debug)]
pub struct TimeSet {
    pub times: Vec<f64>
}

/// A struct representing the differences between adjacent times in a time set.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct TimeSetShape {
    pub intervals: Vec<f64>
}

//-----------------------------------------------
//-------------------- SCALE --------------------
//-----------------------------------------------

/// A struct representing a set of time classes.
/// 
/// ## Predicates
/// 
/// * Time classes must be in ascending order.
/// * Time classes & modulus must be non-negative.
/// * Time classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct TimeClassSet {
    pub time_classes: Vec<f64>,
    pub modulus: f64
}

/// A struct representing a patterned mapping from indices to times.
/// 
/// ## Predicates
/// 
/// * Harmonics must be positive, unique, and in ascending order.
#[derive(PartialEq, Debug)]
pub struct TimeScaleMap {
    pub harmonics: Vec<f64>,
    pub offset: f64
}

/// A struct representing an indexed time class set.
/// 
/// ## Predicates
/// 
/// * Time classes and modulus must be non-negative.
/// * Time classes must be unique. 
/// * Time classes must be less than the modulus. 
/// * Time classes must be in cyclically ascending order.
#[derive(PartialEq, Debug)]
pub struct TimeScaleKey {
    pub time_classes: Vec<f64>,
    pub modulus: f64
}

/// A struct representing the shape of a scale.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct TimeScaleShape {
    pub intervals: Vec<f64>
}

//---------------------------------------------//

pub mod constructors {
    use super::*;

    impl TimeSet {
        pub fn new(times: Vec<f64>) -> Self {
            let mut times = times.clone();
            times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            times.dedup();
    
            Self { times }
        }
    }

    impl TimeSetShape {
        pub fn new(intervals: Vec<f64>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0.0, "Intervals must be positive.");
                }
            }
    
            Self { intervals }
        }
    }

    impl TimeClassSet {
        pub fn new(time_classes: Vec<f64>, modulus: f64) -> Self {
            #[cfg(debug_assertions)]
            {
                for &time_class in time_classes.iter() {
                    assert!(time_class < modulus, "Time classes in TimeClassSet must be less than the modulus.");
                    assert!(floats_are_sorted(&time_classes), "Time classes in TimeClassSet must be in ascending order.");
                    assert!(time_class >= 0.0, "Time classes in TimeClassSet must be non-negative.");
                }
                assert!(modulus >= 0.0, "Modulus of TimeClassSet must be non-negative.");
            }
    
            Self { time_classes, modulus }
        }
    }

    impl TimeScaleMap {
        pub fn new(harmonics: Vec<f64>, offset: f64) -> Self {
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

    impl TimeScaleKey {
        pub fn new(time_classes: Vec<f64>, modulus: f64, root: f64) -> Self {
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
    
            Self { time_classes, modulus }
        }
    }

    impl TimeScaleShape {
        pub fn new(intervals: Vec<f64>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0.0, "Intervals in TimeScaleShape must be positive.");
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
    fn test_time_shape() {
        let time_set_shape = TimeSetShape::new(vec![-1.0, 0.0, 1.2]);
    }

    #[test]
    #[should_panic]
    fn test_time_class_set() {
        let time_class_set = TimeClassSet::new(vec![-1.0, 0.1, 3.2], 2.4);
    }

    #[test]
    #[should_panic]
    fn test_time_scale_map() {
        let time_scale_map = TimeScaleMap::new(vec![-1.2, 3.2, 1.32], 1.2);
    }

    #[test]
    #[should_panic]
    fn test_time_scale_key() {
        let time_scale_key = TimeScaleKey::new(vec![1.2, 3.3, 4.5], 6.0, 2.2);
    }

    #[test]
    #[should_panic]
    fn test_time_scale_shape() {
        let time_scale_shape = TimeScaleShape::new(vec![-1.0, 1.4, 0.23, 0.11]);
    }
}