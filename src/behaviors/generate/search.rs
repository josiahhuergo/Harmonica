use crate::types::{chord::*, melody::*, scale::*, progression::*};
use crate::behaviors::analyze::*;
use itertools::Itertools;

pub mod chord {
    use super::*;

    pub fn scale_chords_in_proximity(chord: Chord, scale: Scale, proximity: i16) -> Vec<Chord> {
        let mut possible_pitches = Vec::<Vec<i16>>::new();

        for pitch in chord.pitches {
            let pitches: Vec<i16> = ((pitch - proximity)..=(pitch + proximity))
                .filter(|&pitch| scale.has_pitch(pitch))
                .collect();
            possible_pitches.push(pitches);
        }

        let chords: Vec<Chord> = possible_pitches.into_iter()
            .multi_cartesian_product()
            .map(|vec| Chord::new(vec))
            .collect();

        chords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod chord {
        use crate::behaviors::generate::search::chord::scale_chords_in_proximity;

        use super::*;

        #[test]
        fn test_scale_chords_in_proximity() {
            let chord = Chord::new(vec![0,6,9]);
            let scale = Scale::new(vec![0,6,9], 12);

            let new_chords = scale_chords_in_proximity(chord, scale, 3);
            println!("{:?}", new_chords);
        }
    }
}