use crate::types::{scale::*, chord::*, melody::*, progression::*, rhythm::*};
use crate::utility::*;
use std::ops::{Sub, Rem, Add, Neg, Mul, AddAssign};
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

/// A trait representing the difference between the lowest and highest pitch in a struct.
pub trait Span {
    fn span(&self) -> i16;
}

/// A trait representing the retrieval of a modulus.
/// "Modulus" refers to two different concepts: a residue class modulus,
/// like in a Scale, or a melodic modulus, like in an interval cycle.
/// 
/// In cases where a melodic cycle contains residue classes, this trait will
/// obtain the residue class modulus, while MelodicModulus will obtain the melodic modulus.
pub trait Modulus<T> {
    fn modulus(&self) -> T;
}

/// A trait representing the retrieval of a melodic modulus. 
/// The melodic modulus is the interval traversed upon completion of a given melodic cycle.
/// 
/// For example, if the cycle, in intervals, is [4,-3], the melodic modulus is 1.
/// This is because, once you complete this cycle, you end up 1 semitone higher than 
/// when you began.
/// 
/// By default, the Modulus trait retrieves this information from melodic cycle types. 
/// The MelodicModulus trait exists for cases where a struct contains residue classes, which
/// have their own notion of modulus, and are also melodic cycles. In this case, Modulus
/// will retrieve the residue class modulus, and MelodicModulus will retrieve the melodic modulus.
pub trait MelodicModulus {
    fn melodic_modulus(&self) -> i16;
}

/// A trait representing the retrieval of a struct's shape.
/// 
/// The shape of an object refers to the distances between adjacent elements.
/// For example, the shape of a chord is the list of differences between adjacent pitches.
/// Or, in a scale, the shape is the list of differences between adjacent pitch classes.
/// 
/// In the former example, the elements are laid out in a linear manner. In the former example,
/// the elements are laid out in a cyclical manner, such that the difference between the first and 
/// last elements are included in the shape.
pub trait Shape {
    type Output;
    fn shape(&self) -> Self::Output;
}

/// A trait representing the stamping of a shape.
/// 
/// This can be considered the inverse of retrieving a struct's shape.
/// It requires a starting element to stamp to. For example, if a chord shape is [4,3], then
/// stamping this onto the pitch 3 will yield the chord [3,7,10].
/// 
/// It's called stamp because it's sort of like stamping the shape onto the linear/cyclical number space.
pub trait Stamp<T> {
    type Output;
    fn stamp(&self, start: T) -> Self::Output;
}

/// A trait pertaining to a cyclical struct's periodicity.
/// 
/// Such structs are either in prime or compound form. 
/// A struct is prime if it is aperiodic, and compound if it is periodic.
/// 
/// The clearest example of this is with an interval cycle. Take [2,3,2,3] - it is clear that
/// there is a repeating substring, [2,3], occurring twice. This makes [2,3,2,3] compound, and [2,3]
/// is its prime form.
/// 
/// It is less obvious when working with pitch structs as opposed to intervals - the scale [0,2,3,5] mod 6
/// is compound, but you don't realize this until you take its shape, [2,1,2,1].
/// 
/// The term "prime form" is not to be confused with prime form in musical set theory.
pub trait Prime<T>: Modulus<T> {
    /// Gets the prime form of the struct.
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

/// A trait representing the evaluation of an indexed struct.
pub trait Eval<T>:  {
    /// Evaluates the scale at index n.
    fn eval(&self, input: T) -> T;
}

/// A trait representing the retrieval of a residue class collection from a collection of numbers.
/// 
/// This is how you acquire the scale that a chord belongs to, or the melody class that a melody belongs to.
pub trait Classify<T> {
    type Output;
    fn classify(&self, modulus: T) -> Self::Output;
}

pub trait HasPitch {
    fn has_pitch(&self, pitch: i16) -> bool;
}

pub mod individual {
    use super::*;

    impl ScaleKey {
        pub fn root(&self) -> i16 {
            *self.pitch_classes.first().unwrap()
        }
    }

    // impl TimeScaleKey {
    //     pub fn root(&self) -> f64 {
    //         *self.time_classes.first().unwrap()
    //     }
    // }
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

    impl Len for Scale {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }

    impl Len for ScaleMap {
        fn len(&self) -> usize {
            self.harmonics.len()
        }
    }

    impl Len for ScaleKey {
        fn len(&self) -> usize {
            self.pitch_classes.len()
        }
    }

    impl Len for ScaleShape {
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

    impl Len for ChordSequence {
        fn len(&self) -> usize {
            self.chords.len()
        }
    }

    impl Len for ScaleSequence {
        fn len(&self) -> usize {
            self.scales.len()
        }
    }

    impl Len for KeySequence {
        fn len(&self) -> usize {
            self.keys.len()
        }
    }

    impl Len for ChordCycle {
        fn len(&self) -> usize {
            self.chords.len()
        }
    }

    impl Len for ScaleCycle {
        fn len(&self) -> usize {
            self.scales.len()
        }
    }

    impl Len for KeyCycle {
        fn len(&self) -> usize {
            self.keys.len()
        }
    }

    // impl Len for TimeSet {
    //     fn len(&self) -> usize {
    //         self.times.len()
    //     }
    // }

    // impl Len for TimeSetShape {
    //     fn len(&self) -> usize {
    //         self.intervals.len()
    //     }
    // }
    
    // impl Len for TimeClassSet {
    //     fn len(&self) -> usize {
    //         self.time_classes.len()
    //     }
    // }

    // impl Len for TimeScaleMap {
    //     fn len(&self) -> usize {
    //         self.harmonics.len()
    //     }
    // }

    // impl Len for TimeScaleKey {
    //     fn len(&self) -> usize {
    //         self.time_classes.len()
    //     }
    // }

    // impl Len for TimeScaleShape {
    //     fn len(&self) -> usize {
    //         self.intervals.len()
    //     }
    // }
}

pub mod span {
    use super::*;

    impl Span for Chord {
        fn span(&self) -> i16 {
            let max = self.pitches.iter().max().unwrap();
            let min = self.pitches.iter().min().unwrap();
            max - min
        }
    }

    impl Span for ChordShape {
        fn span(&self) -> i16 {
            self.intervals.iter().sum()
        }
    }

    impl Span for Melody {
        fn span(&self) -> i16 {
            let max = self.pitches.iter().max().unwrap();
            let min = self.pitches.iter().min().unwrap();
            max - min
        }
    }

    impl Span for MelodyShape {
        fn span(&self) -> i16 {
            self.intervals.iter().sum()
        }
    }
}

pub mod modulus {
    use super::*;

    impl Modulus<i16> for Scale {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for ScaleMap {
        fn modulus(&self) -> i16 {
            *self.harmonics.last().unwrap()
        }
    }

    impl Modulus<i16> for ScaleKey {
        fn modulus(&self) -> i16 {
            self.modulus
        }
    }

    impl Modulus<i16> for ScaleShape
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

    // impl Modulus<f64> for TimeClassSet {
    //     fn modulus(&self) -> f64 {
    //         self.modulus
    //     }
    // }

    // impl Modulus<f64> for TimeScaleMap {
    //     fn modulus(&self) -> f64 {
    //         *self.harmonics.last().unwrap()
    //     }
    // }

    // impl Modulus<f64> for TimeScaleKey {
    //     fn modulus(&self) -> f64 {
    //         self.modulus
    //     }
    // }

    // impl Modulus<f64> for TimeScaleShape
    // {
    //     fn modulus(&self) -> f64 {
    //         self.intervals.iter().cloned().sum()
    //     }
    // }
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

    impl Shape for Scale {
        type Output = ScaleShape;

        fn shape(&self) -> Self::Output {
            let mut intervals = Vec::<i16>::new();
            if self.len() == 1 {
                intervals = vec![self.modulus()];
            } else {
                intervals = self.pitch_classes
                .iter()
                .zip(self.pitch_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            }
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for ScaleMap {
        type Output = ScaleShape;

        fn shape(&self) -> Self::Output {
            let mut intervals: Vec<i16> = self.harmonics
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            intervals.insert(0, self.harmonics[0]);
            
            Self::Output::new(intervals)
        }
    }

    impl Shape for ScaleKey {
        type Output = ScaleShape;

        fn shape(&self) -> Self::Output {
            let mut intervals = Vec::<i16>::new();
            if self.len() == 1 {
                intervals = vec![self.modulus()];
            } else {
                intervals = self.pitch_classes
                .iter()
                .zip(self.pitch_classes.iter().cycle().skip(1))
                .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
                .collect();
            }
            
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

    // impl Shape for TimeSet {
    //     type Output = TimeSetShape;

    //     fn shape(&self) -> Self::Output {
    //         let intervals = self.times
    //             .windows(2)
    //             .map(|window| window[1] - window[0])
    //             .collect();
            
    //         Self::Output::new(intervals)
    //     }
    // }

    // impl Shape for TimeClassSet {
    //     type Output = TimeScaleShape;

    //     fn shape(&self) -> Self::Output {
    //         let intervals = self.time_classes
    //             .iter()
    //             .zip(self.time_classes.iter().cycle().skip(1))
    //             .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
    //             .collect();
            
    //         Self::Output::new(intervals)
    //     }
    // }

    // impl Shape for TimeScaleMap {
    //     type Output = TimeScaleShape;

    //     fn shape(&self) -> Self::Output {
    //         let mut intervals: Vec<f64> = self.harmonics
    //             .windows(2)
    //             .map(|window| window[1] - window[0])
    //             .collect();
    //         intervals.insert(0, self.harmonics[0]);
            
    //         Self::Output::new(intervals)
    //     }
    // }

    // impl Shape for TimeScaleKey {
    //     type Output = TimeScaleShape;

    //     fn shape(&self) -> Self::Output {
    //         let intervals = self.time_classes
    //             .iter()
    //             .zip(self.time_classes.iter().cycle().skip(1))
    //             .map(|(&curr, &next)| (next - curr).rem_euclid(self.modulus()))
    //             .collect();
            
    //         Self::Output::new(intervals)
    //     }
    // }
}

pub mod stamp {
    use super::*;

    impl ScaleShape {
        pub fn stamp_to_scale_map(&self, transposition: i16) -> ScaleMap {
            let harmonics = self.intervals.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }).collect();
    
            ScaleMap::new(harmonics, transposition)
        }
        
        pub fn stamp_to_scale_key(&self, root: i16) -> ScaleKey {
            let scale = self.stamp(root);

            ScaleKey::new(scale.pitch_classes, self.modulus(), root)
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

    impl Stamp<i16> for ScaleShape {
        type Output = Scale;

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

    // impl TimeScaleShape {
    //     pub fn stamp_to_scale_map(&self, offset: f64) -> TimeScaleMap {
    //         let harmonics = self.intervals.iter().scan(0.0, |acc, &x| {
    //             *acc += x;
    //             Some(*acc)
    //         }).collect();
    
    //         TimeScaleMap::new(harmonics, offset)
    //     }
        
    //     pub fn stamp_to_scale_key(&self, root: f64) -> TimeScaleKey {
    //         let time_class_set = self.stamp(root);

    //         TimeScaleKey::new(time_class_set.time_classes, self.modulus(), root)
    //     }
    // }

    // impl Stamp<f64> for TimeSetShape {
    //     type Output = TimeSet;

    //     fn stamp(&self, start: f64) -> Self::Output {
    //         let numbers = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
    //             let next_value = *acc.last().unwrap() + diff;
    //             acc.push(next_value);
    //             acc
    //         });
    
    //         Self::Output::new(numbers)
    //     }
    // }

    // impl Stamp<f64> for TimeScaleShape {
    //     type Output = TimeClassSet;

    //     fn stamp(&self, start: f64) -> Self::Output {
    //         #[cfg(debug_assertions)]
    //         {
    //             assert!(start < self.modulus(), "Starting time class must be less than modulus.");
    //             assert!(start >= 0.0, "Starting time class must be non-negative.");
    //         }

    //         let time_classes: Vec<f64> = std::iter::once(start)
    //             .chain(self.intervals.iter().take(self.len() - 1).scan(start, |acc, &diff| {
    //                 *acc += diff;
    //                 Some(*acc)
    //             }))
    //             .collect();
    
    //         let time_classes: Vec<f64> = time_classes.iter()
    //             .map(|num| (*num).rem_euclid(self.modulus()))
    //             .collect();

    //         let mut time_classes = time_classes.clone();
    //         time_classes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
    //         Self::Output::new(time_classes, self.modulus())
    //     }
    // }
}

pub mod prime {
    use super::*;

    impl Prime<i16> for Scale {
        fn prime(&self) -> Self {
            let smallest_pitch_class = self.pitch_classes.iter().min().cloned().unwrap();
            self.shape().prime().stamp(smallest_pitch_class)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for ScaleMap {
        fn prime(&self) -> Self {
            self.shape().prime().stamp_to_scale_map(self.transposition)
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for ScaleKey {
        fn prime(&self) -> Self {
            let prime = self.shape().prime();
            prime.stamp_to_scale_key(self.root().rem_euclid(prime.modulus()))
        }

        fn is_prime(&self) -> bool {
            self.shape().is_prime()
        }
    }

    impl Prime<i16> for ScaleShape {
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

    // impl Prime<f64> for TimeClassSet {
    //     fn prime(&self) -> Self {
    //         let smallest_time_class = self.time_classes.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned().unwrap();
    //         self.shape().prime().stamp(smallest_time_class)
    //     }

    //     fn is_prime(&self) -> bool {
    //         self.shape().is_prime()
    //     }
    // }

    // impl Prime<f64> for TimeScaleMap {
    //     fn prime(&self) -> Self {
    //         self.shape().prime().stamp_to_scale_map(self.offset)
    //     }

    //     fn is_prime(&self) -> bool {
    //         self.shape().is_prime()
    //     }
    // }

    // impl Prime<f64> for TimeScaleKey {
    //     fn prime(&self) -> Self {
    //         self.shape().prime().stamp_to_scale_key(self.root())
    //     }

    //     fn is_prime(&self) -> bool {
    //         self.shape().is_prime()
    //     }
    // }

    // impl Prime<f64> for TimeScaleShape {
    //     fn prime(&self) -> Self {
    //         let intervals = find_aperiodic_substring(&self.intervals);

    //         Self::new(intervals)
    //     }

    //     fn is_prime(&self) -> bool {
    //         let prime = find_aperiodic_substring(&self.intervals);

    //         self.intervals == prime
    //     }
    // }
}

pub mod eval {
    use super::*;

    impl Eval<i16> for ScaleKey {
        fn eval(&self, input: i16) -> i16 {
            self.pitch_classes[(input as usize).rem_euclid(self.len())]
        }
    }

    impl Eval<i16> for ScaleMap {
        /// Evaluates an index using the scale map.
        fn eval(&self, input: i16) -> i16 {
            let mut rmap: Vec<i16> = self.harmonics.clone();
            rmap.insert(0, 0);
            rmap.pop();

            let r = input.rem_euclid(self.len() as i16);
            let q = (input - r) / self.len() as i16;

            q * self.modulus() + rmap[r as usize] + self.transposition
        }
    }

    impl Eval<Chord> for ScaleMap {
        /// Evaluates a generic chord using the scale map.
        fn eval(&self, input: Chord) -> Chord {
            let pitches: Vec<i16> = input.pitches.iter()
                .map(|pitch| self.eval(*pitch))
                .collect();

            Chord::new(pitches)
        }
    }

    impl Eval<Melody> for ScaleMap {
        /// Evaluates a generic melody using the scale map.
        fn eval(&self, input: Melody) -> Melody {
            let pitches: Vec<i16> = input.pitches.iter()
                .map(|pitch| self.eval(*pitch))
                .collect();

            Melody::new(pitches)
        }
    }

    impl Eval<i16> for MelodicMap {
        /// Evaluates the melodic map at a given index.
        fn eval(&self, index: i16) -> i16 {
            let mut rmap: Vec<i16> = self.harmonics.clone();
            rmap.insert(0, 0);
            rmap.pop();

            let r = index.rem_euclid(self.len() as i16);
            let q = (index - r) / self.len() as i16;

            q * self.modulus() + rmap[r as usize] + self.transposition
        }
    }

    // impl Eval<f64> for TimeScaleKey {
    //     fn eval(&self, index: i16) -> f64 {
    //         self.time_classes[(index as usize).rem_euclid(self.len())]
    //     }
    // }

    // impl Eval<f64> for TimeScaleMap {
    //     /// Evaluates the scale map at a given index.
    //     fn eval(&self, index: i16) -> f64 {
    //         let mut rmap: Vec<f64> = self.harmonics.clone();
    //         rmap.insert(0, 0.0);
    //         rmap.pop();

    //         let r = index.rem_euclid(self.len() as i16);
    //         let q = (index - r) / self.len() as i16;

    //         q as f64 * self.modulus() + rmap[r as usize] + self.offset
    //     }
    // }
}

pub mod classify {
    use super::*;

    impl Classify<i16> for Chord {
        type Output = Scale;

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

    impl Classify<i16> for ChordSequence {
        type Output = ScaleSequence;

        fn classify(&self, modulus: i16) -> Self::Output {
            let scales: Vec<Scale> = self.chords
                .iter()
                .map(|chord| chord.classify(modulus))
                .collect();

            Self::Output::new(scales)
        }
    }

    // impl Classify<f64> for TimeSet {
    //     type Output = TimeClassSet;

    //     fn classify(&self, modulus: f64) -> Self::Output {
    //         let time_classes: Vec<f64> = self.times
    //             .iter()
    //             .map(|n| (*n).rem_euclid(modulus))
    //             .collect();

    //         Self::Output::new(time_classes, modulus)
    //     }
    // }
}

pub mod has_pitch {
    use super::*;

    impl HasPitch for Scale {
        fn has_pitch(&self, pitch: i16) -> bool {
            self.pitch_classes.iter()
                .any(|&pitch_class| pitch.rem_euclid(self.modulus()) == pitch_class)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod individual {
        use super::*;

        #[test]
        fn test_scale_key() {
            let scale_key = ScaleKey::new(vec![0,2,3,5,7], 9, 3);

            assert_eq!(scale_key.root(), 3);
        }

        // #[test]
        // fn test_time_scale_key() {
        //     let time_scale_key = TimeScaleKey::new(vec![0.1, 2.5, 3.68, 4.97], 8.2, 3.68);

        //     assert_eq!(time_scale_key.root(), 3.68);
        // }
    }

    mod span {
        use super::*;

        #[test]
        fn test_chord() {
            let chord = Chord::new(vec![-3,5,7,10]);

            assert_eq!(chord.span(), 13);
        }

        #[test]
        fn test_chord_shape() {
            let chord_shape = ChordShape::new(vec![1,5,2,7]);

            assert_eq!(chord_shape.span(), 15);
        }

        #[test]
        fn test_melody() {
            let melody = Melody::new(vec![-8,4,-2,-1,6,4]);

            assert_eq!(melody.span(), 14);
        }

        #[test]
        fn test_melody_shape() {
            let melody_shape = MelodyShape::new(vec![1,5,-4,2,-7,2,7]);

            assert_eq!(melody_shape.span(), 6);
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
        fn test_scale() {
            let scale = Scale::new(vec![0,3,7], 9);
            let pitch_scale_shape = ScaleShape::new(vec![3,4,2]);

            assert_eq!(scale.shape(), pitch_scale_shape);
        }

        #[test]
        fn test_scale_map() {
            let scale_map = ScaleMap::new(vec![2,4,5,7], 2);
            let pitch_scale_shape = ScaleShape::new(vec![2,2,1,2]);

            assert_eq!(scale_map.shape(), pitch_scale_shape);

            let scale_map = ScaleMap::new(vec![7], 2);
            let pitch_scale_shape = ScaleShape::new(vec![7]);

            assert_eq!(scale_map.shape(), pitch_scale_shape);
        }

        #[test]
        fn test_scale_key() {
            let scale_key = ScaleKey::new(vec![0,1,5,7], 9, 5);
            let pitch_scale_shape = ScaleShape::new(vec![2,2,1,4]);

            assert_eq!(scale_key.shape(), pitch_scale_shape);

            let scale_key = ScaleKey::new(vec![0], 9, 0);
            let pitch_scale_shape = ScaleShape::new(vec![9]);

            assert_eq!(scale_key.shape(), pitch_scale_shape);
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
            let pitch_scale_shape = ScaleShape::new(vec![2,6,4]);
            let scale_map = ScaleMap::new(vec![2,8,12], 3);

            assert_eq!(pitch_scale_shape.stamp_to_scale_map(3), scale_map);
        }

        #[test]
        fn test_pitch_to_scale_key() {
            let pitch_scale_shape = ScaleShape::new(vec![2,6,4]);
            let scale_key = ScaleKey::new(vec![3,7,9], 12, 7);

            assert_eq!(pitch_scale_shape.stamp_to_scale_key(7), scale_key);
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
            let pitch_scale_shape = ScaleShape::new(vec![2,6,4]);
            let scale = Scale::new(vec![0,6,10], 12);

            assert_eq!(pitch_scale_shape.stamp(10), scale);
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
        fn test_scale() {
            let scale = Scale::new(vec![0,2,3,5,6,8,9,11], 12);
            let prime = Scale::new(vec![0,2], 3);

            assert_eq!(scale.prime(), prime);
        }

        #[test]
        fn test_scale_map() {
            let scale_map = ScaleMap::new(vec![2,3,5,6], 2);
            let prime = ScaleMap::new(vec![2,3], 2);

            assert_eq!(scale_map.prime(), prime);
        }

        #[test]
        fn test_scale_key() {
            let scale_key = ScaleKey::new(vec![0,1,3,4], 6, 3);
            let prime = ScaleKey::new(vec![0,1], 3, 0);

            assert_eq!(scale_key.prime(), prime);
        }

        #[test]
        fn test_pitch_scale_shape() {
            let pitch_scale_shape = ScaleShape::new(vec![2,5,4,2,5,4]);
            let prime = ScaleShape::new(vec![2,5,4]);

            assert_eq!(pitch_scale_shape.prime(), prime);
        }
    }

    mod eval {
        use super::*;

        #[test]
        fn test_scale_key() {
            let scale_key = ScaleKey::new(vec![2,3,6,7,9], 12, 6);
            
            assert_eq!(scale_key.eval(3), 2);
        }

        #[test]
        fn test_scale_map_index() {
            let scale_map = ScaleMap::new(vec![2,3,5,7], 3);

            assert_eq!(scale_map.eval(8), 17);
        }

        #[test]
        fn test_scale_map_chord() {
            let scale_map = ScaleMap::new(vec![2,3,5,7], 2);
            let generic_chord = Chord::new(vec![0,2,4]);
            let result = Chord::new(vec![2,5,9]);

            assert_eq!(scale_map.eval(generic_chord), result);
        }

        #[test]
        fn test_scale_map_melody() {
            let scale_map = ScaleMap::new(vec![2,3,5,7], 2);
            let generic_melody = Melody::new(vec![0,-3,2,4]);
            let result = Melody::new(vec![2,-3,5,9]);

            assert_eq!(scale_map.eval(generic_melody), result);
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
            let scale = Scale::new(vec![1,5,8,9], 12);

            assert_eq!(chord.classify(12), scale);
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