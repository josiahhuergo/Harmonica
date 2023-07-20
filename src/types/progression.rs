use crate::types::{chord::*, scale::*};
use crate::behaviors::{analyze::*, transform::*};

use num::integer;

/// A struct representing a sequence of chords.
#[derive(PartialEq, Debug)]
pub struct ChordSequence {
    pub chords: Vec<Chord>
}

/// A struct representing a sequence of pitch class sets.
#[derive(PartialEq, Debug)]
pub struct ScaleSequence {
    pub scales: Vec<Scale>
}

/// A struct representing a sequence of pitch scale keys.
#[derive(PartialEq, Debug)]
pub struct KeySequence { 
    pub keys: Vec<ScaleKey>
}

/// A struct representing a cyclical sequence of chords.
#[derive(PartialEq, Debug)]
pub struct ChordCycle {
    pub chords: Vec<Chord>
}

/// A struct representing a cyclical sequence of pitch class sets.
#[derive(PartialEq, Debug)]
pub struct ScaleCycle {
    pub scales: Vec<Scale>
}

/// A struct representing a cyclical sequence of pitch scale keys.
#[derive(PartialEq, Debug)]
pub struct KeyCycle { 
    pub keys: Vec<ScaleKey>
}

pub mod constructors {
    use super::*;

    impl ChordSequence {
        pub fn new(chords: Vec<Chord>) -> Self {
            Self { chords }
        }
    }

    impl ScaleSequence {
        /// The constructor for a scale progression automatically converts all the scales
        /// to their least common multiple.
        pub fn new(scales: Vec<Scale>) -> Self {
            let moduli: Vec<i16> = scales.iter()
                .map(|scale| scale.modulus())
                .collect();
            let lcm: i16 = moduli.into_iter()
                .fold(1, |acc, x| integer::lcm(acc, x));
            let scales: Vec<Scale> = scales.iter()
                .map(|scale| scale.repeat((lcm / (scale.modulus())) as usize))
                .collect();

            Self { scales }
        }
    }

    impl KeySequence {
        /// The constructor for a scale progression automatically converts all the scales
        /// to their least common multiple.
        pub fn new(keys: Vec<ScaleKey>) -> Self {
            let moduli: Vec<i16> = keys.iter()
                .map(|key| key.modulus())
                .collect();
            let lcm: i16 = moduli.into_iter()
                .fold(1, |acc, x| integer::lcm(acc, x));
            let keys: Vec<ScaleKey> = keys.iter()
                .map(|key| key.repeat((lcm / (key.modulus())) as usize))
                .collect();

            Self { keys }
        }
    }

    impl ChordCycle {
        pub fn new(chords: Vec<Chord>) -> Self {
            Self { chords }
        }
    }

    impl ScaleCycle {
        /// The constructor for a scale progression automatically converts all the scales
        /// to their least common multiple.
        pub fn new(scales: Vec<Scale>) -> Self {
            let moduli: Vec<i16> = scales.iter()
                .map(|scale| scale.modulus())
                .collect();
            let lcm: i16 = moduli.into_iter()
                .fold(1, |acc, x| integer::lcm(acc, x));
            let scales: Vec<Scale> = scales.iter()
                .map(|scale| scale.repeat((lcm / (scale.modulus())) as usize))
                .collect();

            Self { scales }
        }
    }

    impl KeyCycle {
        /// The constructor for a scale progression automatically converts all the scales
        /// to their least common multiple.
        pub fn new(keys: Vec<ScaleKey>) -> Self {
            let moduli: Vec<i16> = keys.iter()
                .map(|key| key.modulus())
                .collect();
            let lcm: i16 = moduli.into_iter()
                .fold(1, |acc, x| integer::lcm(acc, x));
            let keys: Vec<ScaleKey> = keys.iter()
                .map(|key| key.repeat((lcm / (key.modulus())) as usize))
                .collect();

            Self { keys }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_sequence() {
        let scale_sequence = ScaleSequence::new(vec![
            Scale::new(vec![0], 2),
            Scale::new(vec![1,2], 3),
            Scale::new(vec![2,3], 4)
        ]);

        println!("{:?}", scale_sequence);
    }
}