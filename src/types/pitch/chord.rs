use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitches.
pub type PitchSet = Set<i16>;

/// A struct representing the shape of a `PitchSet`.
pub type PitchShape = Shape<i16>;

pub mod constructors {
    use super::*;

    impl PitchSet {
        pub fn new(pitches: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                assert!(collection_is_unique(&pitches), "Pitches in PitchSet must be unique.");
                assert!(collection_is_sorted(&pitches), "Pitches in PitchSet must be sorted.");
            }
    
            Self { numbers: pitches }
        }
    }

    impl ConstructShape<i16> for PitchShape {
        fn new(intervals: Vec<i16>) -> Self {
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