use crate::types::{chord::*, melody::*, scale::*, progression::*};

/// A struct representing a search.
pub struct Search<T> {
    pub results: Vec<T>,
    pub filters: Vec<Filter>
}

/// An enum representing search filter.
pub enum Filter {    
}

// Chord Search
pub type FindChord = Search<Chord>;
pub type FindChordShape = Search<ChordShape>;

// Melody Search
pub type FindMelody = Search<Melody>;
pub type FindMelodyShape = Search<MelodyShape>;
pub type FindMelodyClass = Search<MelodyClass>;
pub type FindMelodyClassShape = Search<MelodyClassShape>;
pub type FindMelodicMap = Search<MelodicMap>;
pub type FindPitchCycle = Search<PitchCycle>;
pub type FindIntervalCycle = Search<IntervalCycle>;
pub type FindPitchClassCycle = Search<PitchClassCycle>;
pub type FindIntervalClassCycle = Search<IntervalClassCycle>;

// Scale Search
pub type FindScale = Search<Scale>;
pub type FindScaleMap = Search<ScaleMap>;
pub type FindScaleKey = Search<ScaleKey>;
pub type FindScaleShape = Search<ScaleShape>;

// Progression Search
pub type FindChordSequence = Search<ChordSequence>;
pub type FindScaleSequence = Search<ScaleSequence>;
pub type FindKeySequence = Search<KeySequence>;
pub type FindChordCycle = Search<ChordCycle>;
pub type FindScaleCycle = Search<ScaleCycle>;
pub type FindKeyCycle = Search<KeyCycle>;