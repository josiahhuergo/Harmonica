use crate::types::{*, pitch::{scale::*, chord::*, melody::*}, rhythm::*};
use crate::utility::*;
use std::ops::{Sub, Rem, Add, Neg, Mul, AddAssign};
use num_traits::{Num, Zero, zero};
use std::iter::Sum;
use std::cmp::{PartialEq, Ord, PartialOrd};

/// Analyze Pitch Module
/// 
/// The `analyze_pitch` module provides behaviors related to the analysis of pitch types.
pub mod analyze_pitch;

/// A trait representing the retrieval of a modulus.
pub trait Modulus<T> {
    /// Get the modulus of the struct.
    fn modulus(&self) -> T;
}

/// A trait representing ways you can analyze a collection.
pub trait Len {
    /// Returns the number of elements in the collection.
    fn len(&self) -> usize; 

    /// Reports whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A trait representing the retrieval of a struct's "shape".
pub trait GetShape<T, U> {
    fn shape(&self) -> U;
}

/// A trait representing the stamping of a "shape".
pub trait StampShape<T, U> {
    fn stamp(&self, start: T) -> U;
}

/// A trait representing the ways you can analyze a scale.
pub trait AnalyzePrimality<T>: Modulus<T> {
    /// Gets the prime subscale.
    fn get_prime(&self) -> Self;

    /// Reports whether the scale is prime (aperiodic).
    fn is_prime(&self) -> bool;
}

/// A trait representing the counting of a scale's modes.
pub trait CountModes<T>: AnalyzePrimality<T> + Len
where
    Self: Sized
{
    /// Counts the number of unique modes in the scale.
    fn count_modes(&self) -> usize {
        self.get_prime().len()
    }
}

/// A trait representing the evaluation of an indexed scale.
pub trait Eval<T>:  {
    /// Evaluates the scale at index n.
    fn eval(&self, index: i16) -> T;
}

/// A trait representing the retrieval of a residue class collection from a collection of numbers.
pub trait Classify<T> {
    type Output;
    fn classify(&self, modulus: T) -> Self::Output;
}

pub mod individual {
    use super::*;

    impl<T: Copy> IndexedResidues<T> {
        pub fn root(&self) -> T {
            *self.residue_classes.first().unwrap()
        }
    }
}

pub mod len {
    use super::*;

    impl<T> Len for Set<T> {
        fn len(&self) -> usize {
            self.numbers.len()
        }
    }
    
    impl<T> Len for Shape<T> {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
    
    impl<T> Len for ResidueSet<T> {
        fn len(&self) -> usize {
            self.residue_classes.len()
        }
    }
    
    impl<T> Len for ScaleMap<T> {
        fn len(&self) -> usize {
            self.harmonics.len()
        }
    }
    
    impl<T> Len for IndexedResidues<T> {
        fn len(&self) -> usize {
            self.residue_classes.len()
        }
    }
    
    impl<T> Len for ScaleShape<T> {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
}

pub mod modulus {
    use super::*;

    impl<T: Copy> Modulus<T> for ResidueSet<T> {
        fn modulus(&self) -> T {
            self.modulus
        }
    }
    
    impl<T: Copy> Modulus<T> for ScaleMap<T> {
        fn modulus(&self) -> T {
            *self.harmonics.last().unwrap()
        }
    }
    
    impl<T: Copy> Modulus<T> for IndexedResidues<T> {
        fn modulus(&self) -> T {
            self.modulus
        }
    }

    impl<T> Modulus<T> for ScaleShape<T>
    where
        T: std::ops::Add<Output = T> + Default + Clone + std::iter::Sum,
    {
        fn modulus(&self) -> T {
            self.intervals.iter().cloned().sum()
        }
    }
}

pub mod get_shape {
    use super::*;

    impl<T> GetShape<T, Shape<T>> for Set<T> 
    where 
        T: Sub + Copy,
        Shape<T>: ConstructShape<T>, 
        Vec<T>: FromIterator<<T as Sub>::Output> 
    {
        fn shape(&self) -> Shape<T> {
                let intervals = self.numbers
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            
            Shape::<T>::new(intervals)
        }
    }

    impl<T> GetShape<T, ScaleShape<T>> for ResidueSet<T>
    where 
        T: Sub<Output = T> + Rem<Output = T> + Add<Output = T> + Copy,
        ScaleShape<T>: ConstructScaleShape<T>
    {
        fn shape(&self) -> ScaleShape<T> {
            let m = self.modulus;
            let intervals = self.residue_classes
                .iter()
                .zip(self.residue_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (((next - curr) % m) + m) % m)
                .collect();
            
            ScaleShape::<T>::new(intervals)
        }
    }

    impl<T> GetShape<T, ScaleShape<T>> for ScaleMap<T>
    where T: Sub<Output = T> + Copy, ScaleShape<T>: ConstructScaleShape<T>
    {
        fn shape(&self) -> ScaleShape<T> {
            let mut intervals: Vec<T> = self.harmonics
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            intervals.insert(0, self.harmonics[0]);
            
            ScaleShape::<T>::new(intervals)
        }
    }

    impl<T> GetShape<T, ScaleShape<T>> for IndexedResidues<T>
    where 
        T: Sub<Output = T> + Rem<Output = T> + Add<Output = T> + Copy,
        ScaleShape<T>: ConstructScaleShape<T>
    {
        fn shape(&self) -> ScaleShape<T> {
            let m = self.modulus;
            let intervals = self.residue_classes
                .iter()
                .zip(self.residue_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (((next - curr) % m) + m) % m)
                .collect();
            
            ScaleShape::<T>::new(intervals)
        }
    }
}

pub mod stamp_shape {
    use super::*;

    impl<T> StampShape<T, Set<T>> for Shape<T>
    where T: Add<Output = T> + Copy, Set<T>: ConstructSet<T>
    {
        fn stamp(&self, start: T) -> Set<T> {
            let numbers = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Set::<T>::new(numbers)
        }
    }

    impl<T> ScaleShape<T> 
    where
        T: AddAssign + Default + Copy + PartialOrd + Sub<Output = T> + Rem<Output = T> + Add<Output = T>,
        ScaleMap<T>: ConstructScaleMap<T>,
        ResidueSet<T>: ConstructResidueSet<T>,
        IndexedResidues<T>: ConstructIndexedResidues<T>,
        ScaleShape<T>: Modulus<T>
    {
        pub fn stamp_to_scale_map(&self, offset: T) -> ScaleMap<T> {
            let harmonics = self.intervals.iter().scan(T::default(), |acc, &x| {
                *acc += x;
                Some(*acc)
            }).collect();
    
            ScaleMap::<T>::new(harmonics, offset)
        }
        
        pub fn stamp_to_scale_key(&self, root: T) -> IndexedResidues<T> {
            let residue_set = self.stamp(root);

            IndexedResidues::<T>::new(residue_set.residue_classes, self.modulus(), root)
        }
    }

    impl<T> StampShape<T, ResidueSet<T>> for ScaleShape<T>
    where 
        T: Copy + PartialOrd + AddAssign + Default + Sub<Output = T> + Rem<Output = T> + Add<Output = T>,
        ResidueSet<T>: ConstructResidueSet<T>, 
        ScaleShape<T>: Modulus<T>
    {
        fn stamp(&self, start: T) -> ResidueSet<T> {
            let m = self.modulus();

            #[cfg(debug_assertions)]
            {
                assert!(start < m, "Starting residue class must be less than modulus.");
                assert!(start >= T::default(), "Starting residue class must be non-negative.");
            }

            let residue_classes: Vec<T> = std::iter::once(start)
                .chain(self.intervals.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let residue_classes = residue_classes.iter()
                .map(|num| ((*num % m) + m) % m)
                .collect();
            
            ResidueSet::<T>::new(residue_classes, m)
        }
    }
}

pub mod analyze_primality {
    use super::*;

    impl<T> AnalyzePrimality<T> for ResidueSet<T>
    where 
        T: Copy + Ord + PartialOrd + AddAssign + Default + Sub<Output = T> + Rem<Output = T> + Add<Output = T>,
        ResidueSet<T>: GetShape<T, ScaleShape<T>> + ConstructResidueSet<T>, 
        ScaleShape<T>: AnalyzePrimality<T>
    {
        fn get_prime(&self) -> Self {
            let smallest_pitch_class = self.residue_classes.iter().min().cloned().unwrap();
            self.shape().get_prime().stamp(smallest_pitch_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl<T> AnalyzePrimality<T> for ScaleMap<T>
    where
        T: Copy + Ord + PartialOrd + AddAssign + Default + Sub<Output = T> + Rem<Output = T> + Add<Output = T>,
        ScaleMap<T>: GetShape<T, ScaleShape<T>> + ConstructScaleMap<T>,
        ScaleShape<T>: AnalyzePrimality<T>,
        ResidueSet<T>: ConstructResidueSet<T>,
        IndexedResidues<T>: ConstructIndexedResidues<T>
    {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_map(self.offset)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl<T> AnalyzePrimality<T> for IndexedResidues<T>
    where
        T: Copy + Ord + PartialOrd + AddAssign + Default + Sub<Output = T> + Rem<Output = T> + Add<Output = T>,
        IndexedResidues<T>: GetShape<T, ScaleShape<T>> + ConstructIndexedResidues<T>, 
        ResidueSet<T>: ConstructResidueSet<T>,
        ScaleMap<T>: ConstructScaleMap<T>,
        ScaleShape<T>: AnalyzePrimality<T>
    {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_key(self.root())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl<T> AnalyzePrimality<T> for ScaleShape<T>
    where 
        T: Add<Output = T> + Sub<Output = T> + Default + Clone + Sum + PartialEq,
        ScaleShape<T>: ConstructScaleShape<T>
    {
        fn get_prime(&self) -> Self {
            let intervals = find_aperiodic_substring(&self.intervals);

            Self::new(intervals)
        }

        fn is_prime(&self) -> bool {
            let prime = find_aperiodic_substring(&self.intervals);

            self.intervals == prime
        }
    }
}

pub mod eval {
    use super::*;

    impl<T: Copy> Eval<T> for IndexedResidues<T> {
        fn eval(&self, index: i16) -> T {
            self.residue_classes[(index as usize).rem_euclid(self.len())]
        }
    }

    impl Eval<i16> for PitchScaleMap {
        /// Evaluates the scale map at a given index.
        /// 
        /// # Arguments
        /// 
        /// * `index`: An integer representing the index at which to evaluate the scale map.
        fn eval(&self, index: i16) -> i16 {
            let mut rmap: Vec<i16> = self.harmonics.clone();
            rmap.insert(0, 0);
            rmap.pop();

            let r = index.rem_euclid(self.len() as i16);
            let q = (index - r) / self.len() as i16;

            q * self.modulus() + rmap[r as usize] + self.offset
        }
    }

    impl Eval<f64> for TimeScaleMap {
        /// Evaluates the scale map at a given index.
        /// 
        /// # Arguments
        /// 
        /// * `index`: An integer representing the index at which to evaluate the scale map.
        fn eval(&self, index: i16) -> f64 {
            let mut rmap: Vec<f64> = self.harmonics.clone();
            rmap.insert(0, 0.0);
            rmap.pop();

            let r = index.rem_euclid(self.len() as i16);
            let q = (index - r) / self.len() as i16;

            q as f64 * self.modulus() + rmap[r as usize] + self.offset
        }
    }
}

pub mod classify {
    use super::*;

    impl Classify<i16> for PitchSet {
        type Output = PitchClassSet;

        fn classify(&self, modulus: i16) -> Self::Output {
            let pitch_classes: Vec<i16> = self.numbers
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(pitch_classes, modulus)
        }
    }

    impl Classify<f64> for TimeSet {
        type Output = TimeClassSet;

        fn classify(&self, modulus: f64) -> Self::Output {
            let time_classes: Vec<f64> = self.numbers
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(time_classes, modulus)
        }
    }

    impl Classify<i16> for Melody {
        type Output = MelodyClass;

        fn classify(&self, modulus: i16) -> Self::Output {
            let pitch_classes: Vec<i16> = self.pitches
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(pitch_classes, modulus)
        }
    }

    impl Classify<i16> for MelodyShape {
        type Output = MelodyClassShape;

        fn classify(&self, modulus: i16) -> Self::Output {
            let interval_classes: Vec<i16> = self.intervals
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(interval_classes, modulus)
        }
    }

    impl Classify<i16> for PitchCycle {
        type Output = PitchClassCycle;

        fn classify(&self, modulus: i16) -> Self::Output {
            let pitch_classes: Vec<i16> = self.pitches
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(pitch_classes, modulus)
        }
    }

    impl Classify<i16> for IntervalCycle {
        type Output = IntervalClassCycle;

        fn classify(&self, modulus: i16) -> Self::Output {
            let interval_classes: Vec<i16> = self.intervals
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(interval_classes, modulus)
        }
    }
}