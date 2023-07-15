use crate::types::{*, pitch::scale::*};
use crate::behaviors::analyze::*;
use num::integer::gcd;

impl PitchScaleMap {
    /// Composes two scale maps.
    /// 
    /// # Arguments
    /// 
    /// * `other`: A second scape map to compose the original with.
    /// 
    /// # Returns
    /// 
    /// Returns a new scale map.
    /// 
    /// # Example
    /// 
    /// ```
    /// use harmonica::pitch::scale::ScaleMap;
    /// 
    /// let scalemap1: ScaleMap = ScaleMap::new(vec![2,3,5], 2);
    /// let scalemap2: ScaleMap = ScaleMap::new(vec![1,3], 1);
    /// 
    /// let composition: ScaleMap = scalemap1.compose(&scalemap2);
    /// 
    /// // composition == ScaleMap { harmonics: vec![3,4,7,10,12,15], transposition: 4 }
    /// ```
    pub fn compose(&self, other: &Self) -> Self {
        let new_t = other.eval(self.transposition);
        let new_period = (self.len() * other.len()) as i16 / gcd(self.modulus(), other.modulus());
        let mut pattern: Vec<i16> = vec![];

        for i in 1..=new_period {
            pattern.push(other.eval(self.eval(i)) - new_t);
        }

        Self::new(pattern, new_t)
    }
}