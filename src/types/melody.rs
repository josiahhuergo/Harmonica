/// A struct representing a sequence of pitches.
#[derive(PartialEq, Debug)]
pub struct Melody {
    pub pitches: Vec<i16>
}

/// A struct representing the shape of a sequence of pitches, i.e. the differences between adjacent pitches.
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
    pub modulus: i16
}

/// A struct representing the shape of a sequence of pitch classes.
/// 
/// ## Predicates
/// 
/// * Interval classes & modulus must be non-negative.
/// * Interval classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct MelodyClassShape {
    pub interval_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing arbitrary melodic cycles, convergent or divergent.
#[derive(PartialEq, Debug)]
pub struct MelodicMap {
    pub harmonics: Vec<i16>,
    pub transposition: i16
}

/// A struct representing a convergent melodic cycle.
#[derive(PartialEq, Debug)]
pub struct PitchCycle {
    pub pitches: Vec<i16>
}

/// A struct representing a cyclical sequence of intervals.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct IntervalCycle {
    pub intervals: Vec<i16>
}

/// A struct representing a cyclical sequence of pitch classes.
/// 
/// ## Predicates
/// 
/// * Pitch classes & modulus must be non-negative.
/// * Pitch classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct PitchClassCycle {
    pub pitch_classes: Vec<i16>,
    pub modulus: i16
}

/// A struct representing a cyclical sequence of interval classes.
/// 
/// ## Predicates
/// 
/// * Interval classes & modulus must be non-negative.
/// * Interval classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct IntervalClassCycle {
    pub interval_classes: Vec<i16>,
    pub modulus: i16
}

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

    impl MelodicMap { // Eval, fix stamp 
        pub fn new(harmonics: Vec<i16>, transposition: i16) -> Self {
            Self { harmonics, transposition }
        }
    }

    impl PitchCycle {
        pub fn new(pitches: Vec<i16>) -> Self {
            Self { pitches }
        }
    }

    impl IntervalCycle {
        pub fn new(intervals: Vec<i16>) -> Self {
            Self { intervals }
        }
    }

    impl PitchClassCycle {
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

    impl IntervalClassCycle {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_melody_class() {
        let melody_class = MelodyClass::new(vec![0,3,6,2,-1,6], 5);
    }

    #[test]
    #[should_panic]
    fn test_melody_class_shape() {
        let melody_class_shape = MelodyClassShape::new(vec![-4,2,7,4,-2], 4);
    }
}