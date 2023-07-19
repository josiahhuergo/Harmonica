use crate::types::scale::*;
use crate::behaviors::analyze::*;
use num::integer::gcd;

/// Search Module
/// 
/// The `search` module provides tools for searching for objects that meet provided criteria.
pub mod search;

impl ScaleMap {
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
    /// use harmonica::types::pitch::scale::ScaleMap;
    /// 
    /// let scalemap1 = ScaleMap::new(vec![2,3,5], 2);
    /// let scalemap2 = ScaleMap::new(vec![1,3], 1);
    /// 
    /// let composition = scalemap1.compose(&scalemap2);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose() {
        let scale_map1 = ScaleMap::new(vec![2,3,5], 2);
        let scale_map2 = ScaleMap::new(vec![1,3], 1);
        let result = ScaleMap::new(vec![3,4,7,10,12,15], 4);

        assert_eq!(scale_map1.compose(&scale_map2), result);
    }
}