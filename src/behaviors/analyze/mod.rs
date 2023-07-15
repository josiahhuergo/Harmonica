use crate::types::{*, pitch::{scale::*, chord::*, melody::*}, rhythm::*};
use crate::utility::*;
use std::ops::{Sub, Rem, Add, Neg, Mul, AddAssign};
use num_traits::{Num, Zero, zero};
use std::iter::Sum;
use std::cmp::{PartialEq, Ord, PartialOrd};

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
pub trait Shape {
    type Output;
    fn shape(&self) -> Self::Output;
}

/// A trait representing the stamping of a "shape".
pub trait Stamp<T> {
    type Output;
    fn stamp(&self, start: T) -> Self::Output;
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

/// A trait representing the counting of a pitch scale's transpositions.
pub trait CountTranspositions: AnalyzePrimality<i16>
where
    Self: Sized
{    
    /// Counts the number of unique transpositions in the scale.
    fn count_transpositions(&self) -> usize {
        self.get_prime().modulus() as usize
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

    impl PitchScaleKey {
        pub fn root(&self) -> i16 {
            *self.pitch_classes.first().unwrap()
        }
    }

    impl TimeScaleKey {
        pub fn root(&self) -> f64 {
            *self.time_classes.first().unwrap()
        }
    }
}

pub mod len {
    use super::*;

    impl Len for PitchSet {
        fn len(&self) -> usize {
            self.pitches.len()
        }
    }

    impl Len for PitchSetShape {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }

    impl Len for PitchClassSet {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }

    impl Len for PitchScaleMap {
        fn len(&self) -> usize {
            self.harmonics.len()
        }
    }

    impl Len for PitchScaleKey {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }

    impl Len for PitchScaleShape {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }

    impl Len for Melody {
        fn len(&self) -> usize {
            self.pitches.len()
        }
    }
    
    impl Len for MelodyShape {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
    
    impl Len for MelodyClass {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }
    
    impl Len for MelodyClassShape {
        fn len(&self) -> usize {
            self.interval_classes.len()
        }
    }

    impl Len for TimeSet {
        fn len(&self) -> usize {
            self.times.len()
        }
    }

    impl Len for TimeSetShape {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
    
    impl Len for TimeClassSet {
        fn len(&self) -> usize {
            self.time_classes.len()
        }
    }

    impl Len for TimeScaleMap {
        fn len(&self) -> usize {
            self.harmonics.len()
        }
    }

    impl Len for TimeScaleKey {
        fn len(&self) -> usize {
            self.time_classes.len()
        }
    }

    impl Len for TimeScaleShape {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
}

pub mod modulus {
    use super::*;

    impl Modulus<i16> for PitchClassSet {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for PitchScaleMap {
        fn modulus(&self) -> i16 {
            *self.harmonics.last().unwrap()
        }
    }

    impl Modulus<i16> for PitchScaleKey {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for PitchScaleShape
    {
        fn modulus(&self) -> i16 {
            self.intervals.iter().cloned().sum()
        }
    }

    impl Modulus<i16> for MelodyClass {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for MelodyClassShape {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<f64> for TimeClassSet {
        fn modulus(&self) -> f64 {
            self.modulus
        }
    }

    impl Modulus<f64> for TimeScaleMap {
        fn modulus(&self) -> f64 {
            *self.harmonics.last().unwrap()
        }
    }

    impl Modulus<f64> for TimeScaleKey {
        fn modulus(&self) -> f64 {
            self.modulus
        }
    }

    impl Modulus<f64> for TimeScaleShape
    {
        fn modulus(&self) -> f64 {
            self.intervals.iter().cloned().sum()
        }
    }
}

pub mod shape {
    use super::*;

    impl Shape for PitchSet {
        type Output = PitchSetShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.pitches
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for PitchClassSet {
        type Output = PitchScaleShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.pitch_classes
                .iter()
                .zip(self.pitch_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for PitchScaleMap {
        type Output = PitchScaleShape;

        fn shape(&self) -> Self::Output {
            let mut intervals: Vec<i16> = self.harmonics
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            intervals.insert(0, self.harmonics[0]);
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for PitchScaleKey {
        type Output = PitchScaleShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.pitch_classes
                .iter()
                .zip(self.pitch_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for Melody {
        type Output = MelodyShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.pitches
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for MelodyClass {
        type Output = MelodyClassShape;

        fn shape(&self) -> Self::Output {
            let interval_classes = self.pitch_classes
                .windows(2)
                .map(|w| (w[1] - w[0]).rem_euclid(self.modulus))
                .collect();
            
            Self::Output::new(interval_classes, self.modulus())
        }
    }

    impl Shape for TimeSet {
        type Output = TimeSetShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.times
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for TimeClassSet {
        type Output = TimeScaleShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.time_classes
                .iter()
                .zip(self.time_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for TimeScaleMap {
        type Output = TimeScaleShape;

        fn shape(&self) -> Self::Output {
            let mut intervals: Vec<f64> = self.harmonics
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            intervals.insert(0, self.harmonics[0]);
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for TimeScaleKey {
        type Output = TimeScaleShape;

        fn shape(&self) -> Self::Output {
            let intervals = self.time_classes
                .iter()
                .zip(self.time_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(intervals)
        }
    }
}

pub mod stamp {
    use super::*;

    impl PitchScaleShape {
        pub fn stamp_to_scale_map(&self, transposition: i16) -> PitchScaleMap {
            let harmonics = self.intervals.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }).collect();
    
            PitchScaleMap::new(harmonics, transposition)
        }
        
        pub fn stamp_to_scale_key(&self, root: i16) -> PitchScaleKey {
            let pitch_class_set = self.stamp(root);

            PitchScaleKey::new(pitch_class_set.pitch_classes, self.modulus(), root)
        }
    }

    impl Stamp<i16> for PitchSetShape {
        type Output = PitchSet;

        fn stamp(&self, start: i16) -> Self::Output {
            let numbers = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Self::Output::new(numbers)
        }
    }

    impl Stamp<i16> for PitchScaleShape {
        type Output = PitchClassSet;

        fn stamp(&self, start: i16) -> Self::Output {
            #[cfg(debug_assertions)]
            {
                assert!(start < self.modulus(), "Starting pitch class must be less than modulus.");
                assert!(start >= 0, "Starting pitch class must be non-negative.");
            }

            let pitch_classes: Vec<i16> = std::iter::once(start)
                .chain(self.intervals.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let pitch_classes = pitch_classes.iter()
                .map(|num| (*num).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(pitch_classes, self.modulus())
        }
    }

    impl Stamp<i16> for MelodyShape {
        type Output = Melody;

        fn stamp(&self, start: i16) -> Self::Output {
            let pitches = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Self::Output::new(pitches)
        }
    }

    impl Stamp<i16> for MelodyClassShape {
        type Output = MelodyClass;

        fn stamp(&self, start: i16) -> Self::Output {
            #[cfg(debug_assertions)]
            {
                assert!(start < self.modulus(), "Stamping pitch must be less than modulus.");
            }
    
            let mut pitch_classes: Vec<i16> = std::iter::once(start)
                .chain(self.interval_classes.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            pitch_classes.iter_mut().map(|num| *num = num.rem_euclid(self.modulus())).collect::<Vec<_>>();
            
            Self::Output::new(pitch_classes, self.modulus())
        }
    }

    impl TimeScaleShape {
        pub fn stamp_to_scale_map(&self, offset: f64) -> TimeScaleMap {
            let harmonics = self.intervals.iter().scan(0.0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }).collect();
    
            TimeScaleMap::new(harmonics, offset)
        }
        
        pub fn stamp_to_scale_key(&self, root: f64) -> TimeScaleKey {
            let time_class_set = self.stamp(root);

            TimeScaleKey::new(time_class_set.time_classes, self.modulus(), root)
        }
    }

    impl Stamp<f64> for TimeSetShape {
        type Output = TimeSet;

        fn stamp(&self, start: f64) -> Self::Output {
            let numbers = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Self::Output::new(numbers)
        }
    }

    impl Stamp<f64> for TimeScaleShape {
        type Output = TimeClassSet;

        fn stamp(&self, start: f64) -> Self::Output {
            #[cfg(debug_assertions)]
            {
                assert!(start < self.modulus(), "Starting time class must be less than modulus.");
                assert!(start >= 0.0, "Starting time class must be non-negative.");
            }

            let time_classes: Vec<f64> = std::iter::once(start)
                .chain(self.intervals.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let time_classes = time_classes.iter()
                .map(|num| (*num).rem_euclid(self.modulus()))
                .collect();
            
            Self::Output::new(time_classes, self.modulus())
        }
    }
}

pub mod analyze_primality {
    use super::*;

    impl AnalyzePrimality<i16> for PitchClassSet {
        fn get_prime(&self) -> Self {
            let smallest_pitch_class = self.pitch_classes.iter().min().cloned().unwrap();
            self.shape().get_prime().stamp(smallest_pitch_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<i16> for PitchScaleMap {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_map(self.transposition)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<i16> for PitchScaleKey {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_key(self.root())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<i16> for PitchScaleShape {
        fn get_prime(&self) -> Self {
            let intervals = find_aperiodic_substring(&self.intervals);

            Self::new(intervals)
        }

        fn is_prime(&self) -> bool {
            let prime = find_aperiodic_substring(&self.intervals);

            self.intervals == prime
        }
    }

    impl AnalyzePrimality<f64> for TimeClassSet {
        fn get_prime(&self) -> Self {
            let smallest_time_class = self.time_classes.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned().unwrap();
            self.shape().get_prime().stamp(smallest_time_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<f64> for TimeScaleMap {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_map(self.offset)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<f64> for TimeScaleKey {
        fn get_prime(&self) -> Self {
            self.shape().get_prime().stamp_to_scale_key(self.root())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl AnalyzePrimality<f64> for TimeScaleShape {
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

    impl Eval<i16> for PitchScaleKey {
        fn eval(&self, index: i16) -> i16 {
            self.pitch_classes[(index as usize).rem_euclid(self.len())]
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

            q * self.modulus() + rmap[r as usize] + self.transposition
        }
    }

    impl Eval<f64> for TimeScaleKey {
        fn eval(&self, index: i16) -> f64 {
            self.time_classes[(index as usize).rem_euclid(self.len())]
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
            let pitch_classes: Vec<i16> = self.pitches
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(pitch_classes, modulus)
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

    impl Classify<f64> for TimeSet {
        type Output = TimeClassSet;

        fn classify(&self, modulus: f64) -> Self::Output {
            let time_classes: Vec<f64> = self.times
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            Self::Output::new(time_classes, modulus)
        }
    }
}