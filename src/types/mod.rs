use std::fmt;

/// Pitch Module
/// 
/// The `pitch` module contains types related to pitch.
pub mod pitch;

/// Rhythm Module
/// 
/// The `rhythm` module contains types related to time.
pub mod rhythm;

//-----------------------------------------------
//-------------------- SCALE --------------------
//-----------------------------------------------

/// A struct representing a set of residue classes.
/// 
/// ## Predicates
/// 
/// * Residue classes must be in ascending order.
/// * Residue classes & modulus must be non-negative.
/// * Residue classes must be less than the modulus.
#[derive(PartialEq, Debug)]
pub struct ResidueSet<T> {
    pub residue_classes: Vec<T>,
    pub modulus: T
}

/// A struct representing a patterned mapping from indices to numbers.
/// 
/// ## Predicates
/// 
/// * Harmonics must be positive, unique, and in ascending order.
#[derive(PartialEq, Debug)]
pub struct ScaleMap<T> {
    pub harmonics: Vec<T>,
    pub offset: T
}

/// A struct representing an indexed residue class set.
/// 
/// ## Predicates
/// 
/// * Residue classes and modulus must be non-negative.
/// * Residue classes must be unique. 
/// * Residue classes must be less than the modulus. 
/// * Residue classes must be in cyclically ascending order.
#[derive(PartialEq, Debug)]
pub struct IndexedResidues<T> {
    pub residue_classes: Vec<T>,
    pub modulus: T
}

/// A struct representing the shape of a scale.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct ScaleShape<T> {
    pub intervals: Vec<T>
}

//-----------------------------------------------
//--------------------- SET ---------------------
//-----------------------------------------------

/// A struct representing a set of numbers.
/// 
/// ## Predicates
/// 
/// * Numbers must be unique.
/// * Numbers must be in ascending order.
#[derive(PartialEq, Debug)]
pub struct Set<T> {
    pub numbers: Vec<T>
}

/// A struct representing the differences between adjacent numbers in a set.
/// 
/// ## Predicates
/// 
/// * Intervals must be positive.
#[derive(PartialEq, Debug)]
pub struct Shape<T> {
    pub intervals: Vec<T>
}

/// A trait representing the generic constructor for Set.
pub trait ConstructSet<T> {
    fn new(numbers: Vec<T>) -> Self;
}

/// A trait representing the generic constructor for Shape.
pub trait ConstructShape<T> {
    fn new(intervals: Vec<T>) -> Self;
}

/// A trait representing the generic constructor for ResidueSet.
pub trait ConstructResidueSet<T> {
    fn new(numbers: Vec<T>, modulus: T) -> Self;
} 

/// A trait representing the generic constructor for ScaleMap.
pub trait ConstructScaleMap<T> {
    fn new(harmonics: Vec<T>, offset: T) -> Self;
} 

/// A trait representing the generic constructor for IndexedResidues.
pub trait ConstructIndexedResidues<T> {
    fn new(residue_classes: Vec<T>, modulus: T, root: T) -> Self;
}

/// A trait representing the generic constructor for ScaleShape.
pub trait ConstructScaleShape<T> {
    fn new(intervals: Vec<T>) -> Self;
}

// Display support for debugging

impl<T: fmt::Display> fmt::Display for ResidueSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.residue_classes.iter().map(|e| e.to_string()).collect();
        write!(f, "{} mod {}", elements.join(", "), self.modulus)
    }
}

impl<T: fmt::Display> fmt::Display for ScaleMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.harmonics.iter().map(|e| e.to_string()).collect();
        write!(f, "{:?} + {}", elements, self.offset)
    }
}

impl<T: fmt::Display> fmt::Display for IndexedResidues<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.residue_classes.iter().map(|e| e.to_string()).collect();
        write!(f, "{} mod {}", elements.join(", "), self.modulus)
    }
}

impl<T: fmt::Display> fmt::Display for ScaleShape<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.intervals.iter().map(|e| e.to_string()).collect();
        write!(f, "{:?}", elements)
    }
}

impl<T: fmt::Display> fmt::Display for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numbers_str: Vec<String> = self.numbers.iter().map(|n| n.to_string()).collect();
        write!(f, "[{}]", numbers_str.join(", "))
    }
}

impl<T: fmt::Display> fmt::Display for Shape<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let intervals_str: Vec<String> = self.intervals.iter().map(|n| n.to_string()).collect();
        write!(f, "[{}]", intervals_str.join(", "))
    }
}
