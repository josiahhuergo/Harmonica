use super::*;
use crate::types::pitch::{melody::*, scale::*};

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

pub mod len {
    use super::*;
    use crate::types::pitch::melody::*;

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
}

pub mod modulus {
    use super::*;

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
}

pub mod get_shape {
    use super::*;

    impl GetShape<i16, MelodyShape> for Melody {
        fn shape(&self) -> MelodyShape {
            let intervals = self.pitches
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            
            MelodyShape::new(intervals)
        }
    }

    impl GetShape<i16, MelodyClassShape> for MelodyClass {
        fn shape(&self) -> MelodyClassShape {
            let interval_classes = self.pitch_classes
                .windows(2)
                .map(|w| (w[1] - w[0]).rem_euclid(self.modulus))
                .collect();
            
            MelodyClassShape::new(interval_classes, self.modulus())
        }
    }

    impl GetShape<i16, IntervalCycle> for PitchCycle {
        fn shape(&self) -> IntervalCycle {
            let intervals = (1..self.len())
                .map(|i| self.pitches[i as usize] - self.pitches[i as usize - 1])
                .chain(std::iter::once(self.pitches[0] - self.pitches[self.len() as usize - 1]))
                .collect();

            IntervalCycle::new(intervals)
        }
    }

    impl GetShape<i16, IntervalClassCycle> for PitchClassCycle {
        fn shape(&self) -> IntervalClassCycle {
            let intervals = (1..self.len())
                .map(|i| self.pitch_classes[i as usize] - self.pitch_classes[i as usize - 1])
                .chain(std::iter::once((self.pitch_classes[0] - self.pitch_classes[self.len() as usize - 1]).rem_euclid(self.modulus())))
                .collect();

            IntervalClassCycle::new(intervals, self.modulus())
        }
    }
}

pub mod stamp_shape {
    use super::*;

    impl StampShape<i16, Melody> for MelodyShape {
        fn stamp(&self, start: i16) -> Melody {
            let pitches = self.intervals.iter().fold(vec![start], |mut acc, &diff| {
                let next_value = *acc.last().unwrap() + diff;
                acc.push(next_value);
                acc
            });
    
            Melody::new(pitches)
        }
    }

    impl StampShape<i16, MelodyClass> for MelodyClassShape {
        fn stamp(&self, start: i16) -> MelodyClass {
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
            
            MelodyClass::new(pitch_classes, self.modulus())
        }
    }

    impl StampShape<i16, PitchCycle> for IntervalCycle {
        fn stamp(&self, start: i16) -> PitchCycle {
            let pitches = std::iter::once(start)
                .chain(self.intervals.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .take(self.len() as usize)
                .collect();

            PitchCycle::new(pitches)
        }
    }

    impl StampShape<i16, PitchClassCycle> for IntervalClassCycle {
        fn stamp(&self, start: i16) -> PitchClassCycle {
            let pitch_classes: Vec<i16> = std::iter::once(start)
                .chain(self.interval_classes.iter().scan(start, |acc, &diff| {
                    *acc += diff;
                    Some(*acc)
                }))
                .take(self.len() as usize)
                .collect();

            let pitch_classes: Vec<i16> = pitch_classes.iter()
                .map(|n| n.rem_euclid(self.modulus()))
                .collect();

            PitchClassCycle::new(pitch_classes, self.modulus())
        }
    }
}
