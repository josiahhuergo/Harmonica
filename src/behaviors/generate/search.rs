use crate::types::{chord::*, melody::*, scale::*, progression::*};

/// A struct representing the results of a search.
pub struct SearchResults<T> {
    pub results: Vec<T>
}

// Chord Search
pub type FindChord = SearchResults<Chord>;
pub type FindChordShape = SearchResults<ChordShape>;

// Melody Search
pub type FindMelody = SearchResults<Melody>;
pub type FindMelodyShape = SearchResults<MelodyShape>;
pub type FindMelodyClass = SearchResults<MelodyClass>;
pub type FindMelodyClassShape = SearchResults<MelodyClassShape>;
pub type FindMelodicMap = SearchResults<MelodicMap>;
pub type FindPitchCycle = SearchResults<PitchCycle>;
pub type FindIntervalCycle = SearchResults<IntervalCycle>;
pub type FindPitchClassCycle = SearchResults<PitchClassCycle>;
pub type FindIntervalClassCycle = SearchResults<IntervalClassCycle>;

// Scale Search
pub type FindScale = SearchResults<Scale>;
pub type FindScaleMap = SearchResults<ScaleMap>;
pub type FindScaleKey = SearchResults<ScaleKey>;
pub type FindScaleShape = SearchResults<ScaleShape>;

// Progression Search
pub type FindChordSequence = SearchResults<ChordSequence>;
pub type FindScaleSequence = SearchResults<ScaleSequence>;
pub type FindKeySequence = SearchResults<KeySequence>;
pub type FindChordCycle = SearchResults<ChordCycle>;
pub type FindScaleCycle = SearchResults<ScaleCycle>;
pub type FindKeyCycle = SearchResults<KeyCycle>;