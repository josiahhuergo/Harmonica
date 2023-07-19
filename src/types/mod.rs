use std::fmt;

/// Chord Module
/// 
/// The `chord` module contains types related to chords.
pub mod chord;

/// Melody Module
/// 
/// The `melody` moudle contains types related to melodies.
pub mod melody;

/// Scale Module
/// 
/// The `scale` module contains types related to scales.
pub mod scale;

/// Progression Module
/// 
/// The `progression` module contains types related to chord/scale progressions.
pub mod progression;

/// Rhythm Module
/// 
/// The `rhythm` module contains types related to time.
pub mod rhythm;

// Display support for debugging

// impl<T: fmt::Display> fmt::Display for ResidueSet<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let elements: Vec<String> = self.residue_classes.iter().map(|e| e.to_string()).collect();
//         write!(f, "{} mod {}", elements.join(", "), self.modulus)
//     }
// }

// impl<T: fmt::Display> fmt::Display for ScaleMap<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let elements: Vec<String> = self.harmonics.iter().map(|e| e.to_string()).collect();
//         write!(f, "{:?} + {}", elements, self.offset)
//     }
// }

// impl<T: fmt::Display> fmt::Display for IndexedResidues<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let elements: Vec<String> = self.residue_classes.iter().map(|e| e.to_string()).collect();
//         write!(f, "{} mod {}", elements.join(", "), self.modulus)
//     }
// }

// impl<T: fmt::Display> fmt::Display for ScaleShape<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let elements: Vec<String> = self.intervals.iter().map(|e| e.to_string()).collect();
//         write!(f, "{:?}", elements)
//     }
// }

// impl<T: fmt::Display> fmt::Display for Set<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let numbers_str: Vec<String> = self.numbers.iter().map(|n| n.to_string()).collect();
//         write!(f, "[{}]", numbers_str.join(", "))
//     }
// }

// impl<T: fmt::Display> fmt::Display for Shape<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let intervals_str: Vec<String> = self.intervals.iter().map(|n| n.to_string()).collect();
//         write!(f, "[{}]", intervals_str.join(", "))
//     }
// }
