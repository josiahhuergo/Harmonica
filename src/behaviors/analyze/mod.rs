use crate::types::{*, pitch::{scale::*, chord::*, melody::*}, rhythm::*};
use crate::utility::*;
use std::ops::{Sub, Rem, Add, Neg, Mul, AddAssign};
use num_traits::{Num, Zero, zero};
use std::iter::Sum;
use std::cmp::{PartialEq, Ord, PartialOrd};

/// A trait representing the retrieval of a collection's size.
pub trait Len {
    /// Returns the number of elements in the collection.
    fn len(&self) -> usize; 

    /// Reports whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A trait representing the retrieval of a modulus.
pub trait Modulus<T> {
    /// Get the modulus of the struct.
    fn modulus(&self) -> T;
}

/// A trait reprsenting the retrieval of a melodic modulus.
pub trait MelodicModulus {
    fn melodic_modulus(&self) -> i16;
}

/// A trait representing the retrieval of a struct's shape.
pub trait Shape {
    type Output;
    fn shape(&self) -> Self::Output;
}

/// A trait representing the stamping of a shape.
pub trait Stamp<T> {
    type Output;
    fn stamp(&self, start: T) -> Self::Output;
}

/// A trait representing the ways you can analyze a scale.
pub trait Prime<T>: Modulus<T> {
    /// Gets the prime subscale.
    fn prime(&self) -> Self;

    /// Reports whether the scale is prime (aperiodic).
    fn is_prime(&self) -> bool;
}

/// A trait representing the counting of a scale's modes.
pub trait CountModes<T>: Prime<T> + Len
where
    Self: Sized
{
    /// Counts the number of unique modes in the scale.
    fn count_modes(&self) -> usize {
        self.prime().len()
    }
}

/// A trait representing the counting of a pitch scale's transpositions.
pub trait CountTranspositions: Prime<i16>
where
    Self: Sized
{    
    /// Counts the number of unique transpositions in the scale.
    fn count_transpositions(&self) -> usize {
        self.prime().modulus() as usize
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

    impl Len for Chord {
        fn len(&self) -> usize {
            self.pitches.len()
        }
    }

    impl Len for ChordShape {
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

    impl Len for MelodicMap {
        fn len(&self) -> usize {
            self.harmonics.len()
        }
    }

    impl Len for PitchCycle {
        fn len(&self) -> usize {
            self.pitches.len()
        }
    }
    
    impl Len for IntervalCycle {
        fn len(&self) -> usize {
            self.intervals.len()
        }
    }
    
    impl Len for PitchClassCycle {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }
    
    impl Len for IntervalClassCycle {
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

    impl Modulus<i16> for PitchClassCycle {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for IntervalClassCycle {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for MelodicMap {
        fn modulus(&self) -> i16 {
            *self.harmonics.last().unwrap()
        }
    }

    impl Modulus<i16> for PitchCycle {
        fn modulus(&self) -> i16 {
            0
        }
    }

    impl Modulus<i16> for IntervalCycle {
        fn modulus(&self) -> i16 {
            self.intervals.iter().sum()
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

pub mod melodic_modulus {
    use super::*;

    impl MelodicModulus for PitchClassCycle {
        fn melodic_modulus(&self) -> i16 {
            0
        }
    }

    impl MelodicModulus for IntervalClassCycle {
        fn melodic_modulus(&self) -> i16 {
            self.interval_classes.iter().sum::<i16>().rem_euclid(self.modulus())
        }
    }
}

pub mod shape {
    use super::*;

    impl Shape for Chord {
        type Output = ChordShape;

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

    impl Shape for MelodicMap {
        type Output = IntervalCycle;

        fn shape(&self) -> Self::Output {
            let mut intervals: Vec<i16> = self.harmonics
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            intervals.insert(0, self.harmonics[0]);
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for PitchCycle {
        type Output = IntervalCycle;

        fn shape(&self) -> Self::Output {
            let intervals = self.pitches
                .iter()
                .zip(self.pitches.iter().cycle().skip(1))
                .map(|(&curr, &next)| next - curr)
                .collect();
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for PitchClassCycle {
        type Output = IntervalClassCycle;

        fn shape(&self) -> Self::Output {
            let interval_classes = self.pitch_classes
                .iter()
                .zip(self.pitch_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
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

    impl IntervalCycle {
        pub fn stamp_to_pitch_cycle(&self, pitch: i16) -> PitchCycle {
            #[cfg(debug_assertions)]
            {
                assert_eq!(self.intervals.iter().sum::<i16>(), 0);
            }

            let pitches = self.intervals.iter().take(self.len() - 1).fold(vec![pitch], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });

            PitchCycle::new(pitches)
        }
    }

    impl Stamp<i16> for ChordShape {
        type Output = Chord;

        fn stamp(&self, start: i16) -> Self::Output {
            let pitches = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Self::Output::new(pitches)
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
                .chain(self.intervals.iter().take(self.len() - 1).scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let pitch_classes: Vec<i16> = pitch_classes.iter()
                .map(|num| (*num).rem_euclid(self.modulus()))
                .collect();

            let mut pitch_classes = pitch_classes.clone();
            pitch_classes.sort();
            
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

    impl Stamp<i16> for IntervalCycle {
        type Output = MelodicMap;

        fn stamp(&self, start: i16) -> Self::Output {
            let harmonics = self.intervals.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }).collect();
    
            Self::Output::new(harmonics, start)
        }
    }

    impl Stamp<i16> for IntervalClassCycle {
        type Output = PitchClassCycle;

        fn stamp(&self, start: i16) -> Self::Output {
            let pitch_classes: Vec<i16> = std::iter::once(start)
                .chain(self.interval_classes.iter().take(self.len() - 1).scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let pitches: Vec<i16> = pitch_classes.iter()
                .map(|num| (*num).rem_euclid(self.modulus()))
                .collect();
            
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
                .chain(self.intervals.iter().take(self.len() - 1).scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .collect();
    
            let time_classes: Vec<f64> = time_classes.iter()
                .map(|num| (*num).rem_euclid(self.modulus()))
                .collect();

            let mut time_classes = time_classes.clone();
            time_classes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            Self::Output::new(time_classes, self.modulus())
        }
    }
}

pub mod prime {
    use super::*;

    impl Prime<i16> for PitchClassSet {
        fn prime(&self) -> Self {
            let smallest_pitch_class = self.pitch_classes.iter().min().cloned().unwrap();
            self.shape().prime().stamp(smallest_pitch_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for PitchScaleMap {
        fn prime(&self) -> Self {
            self.shape().prime().stamp_to_scale_map(self.transposition)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for PitchScaleKey {
        fn prime(&self) -> Self {
            let prime = self.shape().prime();
            prime.stamp_to_scale_key(self.root().rem_euclid(prime.modulus()))
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for PitchScaleShape {
        fn prime(&self) -> Self {
            let intervals = find_aperiodic_substring(&self.intervals);

            Self::new(intervals)
        }

        fn is_prime(&self) -> bool {
            let prime = find_aperiodic_substring(&self.intervals);

            self.intervals == prime
        }
    }

    impl Prime<i16> for PitchCycle {
        fn prime(&self) -> Self {
            self.shape().prime().stamp_to_pitch_cycle(*self.pitches.first().unwrap())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for IntervalCycle {
        fn prime(&self) -> Self {
            let prime = find_aperiodic_substring(&self.intervals);

            Self::new(prime)
        }

        fn is_prime(&self) -> bool {
            let prime = find_aperiodic_substring(&self.intervals);

            self.intervals == prime
        }
    }

    impl Prime<i16> for PitchClassCycle {
        fn prime(&self) -> Self {
            self.shape().prime().stamp(*self.pitch_classes.first().unwrap())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for IntervalClassCycle {
        fn prime(&self) -> Self {
            let prime = find_aperiodic_substring(&self.interval_classes);

            Self::new(prime, self.modulus())
        }

        fn is_prime(&self) -> bool {
            let prime = find_aperiodic_substring(&self.interval_classes);

            self.interval_classes == prime
        }
    }

    impl Prime<f64> for TimeClassSet {
        fn prime(&self) -> Self {
            let smallest_time_class = self.time_classes.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned().unwrap();
            self.shape().prime().stamp(smallest_time_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<f64> for TimeScaleMap {
        fn prime(&self) -> Self {
            self.shape().prime().stamp_to_scale_map(self.offset)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<f64> for TimeScaleKey {
        fn prime(&self) -> Self {
            self.shape().prime().stamp_to_scale_key(self.root())
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<f64> for TimeScaleShape {
        fn prime(&self) -> Self {
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

    impl Eval<i16> for MelodicMap {
        /// Evaluates the melodic map at a given index.
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

    impl Classify<i16> for Chord {
        type Output = PitchClassSet;

        fn classify(&self, modulus: i16) -> Self::Output {
            let pitch_classes: Vec<i16> = self.pitches
                .iter()
                .map(|n| (*n).rem_euclid(modulus))
                .collect();

            let mut pitch_classes = pitch_classes.clone();
            pitch_classes.dedup();
            pitch_classes.sort();

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

#[cfg(test)]
mod tests {
    use super::*;

    mod individual {
        use super::*;

        #[test]
        fn test_pitch_scale_key() {
            let pitch_scale_key = PitchScaleKey::new(vec![0,2,3,5,7], 9, 3);

            assert_eq!(pitch_scale_key.root(), 3);
        }

        #[test]
        fn test_time_scale_key() {
            let time_scale_key = TimeScaleKey::new(vec![0.1, 2.5, 3.68, 4.97], 8.2, 3.68);

            assert_eq!(time_scale_key.root(), 3.68);
        }
    }

    mod shape {
        use super::*;

        #[test]
        fn test_chord() {
            let chord = Chord::new(vec![1,4,8,12]);
            let shape = ChordShape::new(vec![3,4,4]);

            assert_eq!(chord.shape(), shape);
        }

        #[test]
        fn test_pitch_class_set() {
            let pitch_class_set = PitchClassSet::new(vec![0,3,7], 9);
            let pitch_scale_shape = PitchScaleShape::new(vec![3,4,2]);

            assert_eq!(pitch_class_set.shape(), pitch_scale_shape);
        }

        #[test]
        fn test_pitch_scale_map() {
            let pitch_scale_map = PitchScaleMap::new(vec![2,4,5,7], 2);
            let pitch_scale_shape = PitchScaleShape::new(vec![2,2,1,2]);

            assert_eq!(pitch_scale_map.shape(), pitch_scale_shape);
        }

        #[test]
        fn test_pitch_scale_key() {
            let pitch_scale_key = PitchScaleKey::new(vec![0,1,5,7], 9, 5);
            let pitch_scale_shape = PitchScaleShape::new(vec![2,2,1,4]);

            assert_eq!(pitch_scale_key.shape(), pitch_scale_shape);
        }

        #[test]
        fn test_melody() {
            let melody = Melody::new(vec![-4,7,1,3,1,1,7,4]);
            let melody_shape = MelodyShape::new(vec![11,-6,2,-2,0,6,-3]);

            assert_eq!(melody.shape(), melody_shape);
        }

        #[test]
        fn test_melody_class() {
            let melody_class = MelodyClass::new(vec![1,6,3,7,2,5,5,3], 9);
            let melody_class_shape = MelodyClassShape::new(vec![5,6,4,4,3,0,7], 9);

            assert_eq!(melody_class.shape(), melody_class_shape);
        }

        #[test]
        fn test_melodic_map() {
            let melodic_map = MelodicMap::new(vec![-2,7,3,2], 3);
            let shape = IntervalCycle::new(vec![-2,9,-4,-1]);

            assert_eq!(melodic_map.shape(), shape);
        }

        #[test]
        fn test_pitch_cycle() {
            let pitch_cycle = PitchCycle::new(vec![2,7,5,1,-5]);
            let interval_cycle = IntervalCycle::new(vec![5,-2,-4,-6,7]);

            assert_eq!(pitch_cycle.shape(), interval_cycle);
        }

        #[test]
        fn test_pitch_class_cycle() {
            let pitch_class_cycle = PitchClassCycle::new(vec![0,4,2,4], 12);
            let interval_class_cycle = IntervalClassCycle::new(vec![4,10,2,8], 12);

            assert_eq!(pitch_class_cycle.shape(), interval_class_cycle);
        }

        // Doesn't work cuz of fucking floats! :3
        // #[test]
        // fn test_time_set() {
        //     let time_set = TimeSet::new(vec![0.4, 1.2, 2.4, 3.33]);
        //     let time_set_shape = TimeSetShape::new(vec![0.8, 1.2, 0.93]);

        //     assert_eq!(time_set.shape(), time_set_shape);
        // }
    }

    mod stamp {
        use super::*;

        #[test]
        fn test_pitch_to_scale_map() {
            let pitch_scale_shape = PitchScaleShape::new(vec![2,6,4]);
            let pitch_scale_map = PitchScaleMap::new(vec![2,8,12], 3);

            assert_eq!(pitch_scale_shape.stamp_to_scale_map(3), pitch_scale_map);
        }

        #[test]
        fn test_pitch_to_scale_key() {
            let pitch_scale_shape = PitchScaleShape::new(vec![2,6,4]);
            let pitch_scale_key = PitchScaleKey::new(vec![3,7,9], 12, 7);

            assert_eq!(pitch_scale_shape.stamp_to_scale_key(7), pitch_scale_key);
        }

        #[test]
        fn test_interval_to_pitch_cycle() {
            let interval_cycle = IntervalCycle::new(vec![2,7,-4,0,-3,4,-6]);
            let result = PitchCycle::new(vec![6,8,15,11,11,8,12]);

            assert_eq!(interval_cycle.stamp_to_pitch_cycle(6), result);
        }

        #[test]
        fn test_chord_shape() {
            let chord_shape = ChordShape::new(vec![2,7,3,5]);
            let chord = Chord::new(vec![4,6,13,16,21]);

            assert_eq!(chord_shape.stamp(4), chord);
        }

        #[test]
        fn test_pitch_scale_shape() {
            let pitch_scale_shape = PitchScaleShape::new(vec![2,6,4]);
            let pitch_class_set = PitchClassSet::new(vec![0,6,10], 12);

            assert_eq!(pitch_scale_shape.stamp(10), pitch_class_set);
        }

        #[test]
        fn test_melody_shape() {
            let melody_shape = MelodyShape::new(vec![2,7,3,-6,5]);
            let melody = Melody::new(vec![4,6,13,16,10,15]);

            assert_eq!(melody_shape.stamp(4), melody);
        }

        #[test]
        fn test_melody_class_shape() {
            let melody_class_shape = MelodyClassShape::new(vec![0,0,5,8], 12);
            let melody_class = MelodyClass::new(vec![2,2,2,7,3], 12);

            assert_eq!(melody_class_shape.stamp(2), melody_class);
        }

        #[test]
        fn test_interval_cycle() {
            let interval_cycle = IntervalCycle::new(vec![3,-6,7,-1,-1]);
            let pitch_cycle = MelodicMap::new(vec![3,-3,4,3,2], 0);

            assert_eq!(interval_cycle.stamp(0), pitch_cycle);
        }

        #[test]
        fn test_interval_class_cycle() {
            let interval_class_cycle = IntervalClassCycle::new(vec![2,5,3,5,9], 12);
            let pitch_class_cycle = PitchClassCycle::new(vec![4,6,11,2,7], 12);

            assert_eq!(interval_class_cycle.stamp(4), pitch_class_cycle);
        }
    }

    mod prime {
        use super::*;

        #[test]
        fn test_pitch_class_set() {
            let pitch_class_set = PitchClassSet::new(vec![0,2,3,5,6,8,9,11], 12);
            let prime = PitchClassSet::new(vec![0,2], 3);

            assert_eq!(pitch_class_set.prime(), prime);
        }

        #[test]
        fn test_pitch_scale_map() {
            let pitch_scale_map = PitchScaleMap::new(vec![2,3,5,6], 2);
            let prime = PitchScaleMap::new(vec![2,3], 2);

            assert_eq!(pitch_scale_map.prime(), prime);
        }

        #[test]
        fn test_pitch_scale_key() {
            let pitch_scale_key = PitchScaleKey::new(vec![0,1,3,4], 6, 3);
            let prime = PitchScaleKey::new(vec![0,1], 3, 0);

            assert_eq!(pitch_scale_key.prime(), prime);
        }

        #[test]
        fn test_pitch_scale_shape() {
            let pitch_scale_shape = PitchScaleShape::new(vec![2,5,4,2,5,4]);
            let prime = PitchScaleShape::new(vec![2,5,4]);

            assert_eq!(pitch_scale_shape.prime(), prime);
        }
    }

    mod eval {
        use super::*;

        #[test]
        fn test_pitch_scale_key() {
            let pitch_scale_key = PitchScaleKey::new(vec![2,3,6,7,9], 12, 6);
            
            assert_eq!(pitch_scale_key.eval(3), 2);
        }

        #[test]
        fn test_pitch_scale_map() {
            let pitch_scale_map = PitchScaleMap::new(vec![2,3,5,7], 3);

            assert_eq!(pitch_scale_map.eval(8), 17);
        }

        #[test]
        fn test_melodic_map() {
            let melodic_map = MelodicMap::new(vec![3,-1,4], 0);

            assert_eq!(melodic_map.eval(7), 11);
        }
    }

    mod classify {
        use super::*;

        #[test]
        fn test_chord() {
            let chord = Chord::new(vec![-3,5,8,25]);
            let pitch_class_set = PitchClassSet::new(vec![1,5,8,9], 12);

            assert_eq!(chord.classify(12), pitch_class_set);
        }

        #[test]
        fn test_melody() {
            let melody = Melody::new(vec![-4,8,6,6,2,7,19]);
            let melody_class = MelodyClass::new(vec![8,8,6,6,2,7,7], 12);

            assert_eq!(melody.classify(12), melody_class);
        }

        #[test]
        fn test_melody_shape() {
            let melody_shape = MelodyShape::new(vec![2,15,-5,4,14,7]);
            let melody_class_shape = MelodyClassShape::new(vec![2,3,7,4,2,7], 12);

            assert_eq!(melody_shape.classify(12), melody_class_shape);
        }
    }
}