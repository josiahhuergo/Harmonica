# Harmonica

Harmonica is a collection of tools for the analysis, generation, and transformation of music theoretical objects, written in rust.

## TO-DO:

* Differentiate between ResidueModulus and CycleModulus
* Tools for finding objects that meet criteria (FindPitchClassSet, FindChord, etc)
* Chord/scale progressions / "PolyScales"
* ScaleMaps but for melodies (to solve the problem of representing "divergent melodic cycles" whose pitch cycles are infinite and can't be represented with a finite vector)
* Maps (like ScaleMaps) that relative_rotate whenever they're evaluated ("RelativeMaps"?)
* Timeline object
* MIDI tools (reading MIDI, writing MIDI)
* Re-export objects for the API (in api.rs)s
* Write more descriptive documentation
* Redesign rhythm tools (floats suck tbh)
