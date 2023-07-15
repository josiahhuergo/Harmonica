use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitches.
/// 
/// ## Predicates
/// 
/// * Pitches must be unique.
/// * Pitches must be in ascending order.
#[derive(PartialEq, Debug)]
pub struct PitchSet {
    pub pitches: Vec<i16>
}

/// A struct representing the differences between adjacent pitches in a pitch set.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct PitchSetShape {
    pub intervals: Vec<i16>
}

pub mod constructors {
    use super::*;

    impl PitchSet {
        pub fn new(pitches: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                assert!(collection_is_unique(&pitches), "Pitches in PitchSet must be unique.");
                assert!(collection_is_sorted(&pitches), "Pitches in PitchSet must be sorted.");
            }
    
            Self { pitches }
        }
    }

    impl PitchSetShape {
        pub fn new(intervals: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0, "Intervals in PitchShape must be positive.");
                }
            }
    
            Self { intervals }
        }
    }
}