use crate::types::*;
use crate::utility::*;

/// A struct representing a set of pitches.
/// 
/// ## Predicates
/// 
/// * Pitches must be unique.
/// * Pitches must be in ascending order.
#[derive(PartialEq, Debug)]
pub struct Chord {
    pub pitches: Vec<i16>
}

/// A struct representing the differences between adjacent pitches in a pitch set.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct ChordShape {
    pub intervals: Vec<i16>
}

pub mod constructors {
    use super::*;

    impl Chord {
        pub fn new(pitches: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                assert!(collection_is_unique(&pitches), "Pitches in Chord must be unique.");
                assert!(collection_is_sorted(&pitches), "Pitches in Chord must be sorted.");
            }
    
            Self { pitches }
        }
    }

    impl ChordShape {
        pub fn new(intervals: Vec<i16>) -> Self {
            #[cfg(debug_assertions)]
            {
                for &interval in intervals.iter() {
                    assert!(interval > 0, "Intervals in ChordShape must be positive.");
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
    fn test_chord() {
        let chord = Chord::new(vec![0, 0, -3, 4]);
    }

    #[test]
    #[should_panic]
    fn test_chord_shape() {
        let chord_shape = ChordShape::new(vec![-2,4,2,3]);
    }
}