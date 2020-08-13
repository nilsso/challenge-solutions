#![feature(bool_to_option)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

mod accumulate;
pub use accumulate::Accumulate;

#[derive(Debug)]
pub enum Error {
    InvalidPitch(String),
    InvalidIntervals(String),
}

// Easy lookup tables for going between pitch class values and pitch names.
lazy_static! {
    static ref NAME_TO_VALUE: HashMap<&'static str, u16> = hashmap! {
        "c"  => 0, "c#" => 1, "db" => 1, "d"  => 2, "d#" => 3, "eb" => 3, "e"  => 4,
        "f"  => 5, "f#" => 6, "gb" => 6, "g"  => 7, "g#" => 8, "ab" => 8, "a"  => 9,
        "a#" => 10, "bb" => 10, "b"  => 11,
    };
    static ref VALUE_TO_SHARP_NAMES: HashMap<u16, &'static str> = hashmap! {
        0 => "C", 1 => "C#", 2 => "D", 3 => "D#", 4 => "E", 5 => "F", 6 => "F#",
        7 => "G", 8 => "G#", 9 => "A", 10 => "A#", 11 => "B",
    };
    static ref VALUE_TO_FLAT_NAMES: HashMap<u16, &'static str> = hashmap! {
        0 => "C", 1 => "Db", 2 => "D", 3 => "Eb", 4 => "E", 5 => "F", 6 => "Gb",
        7 => "G", 8 => "Ab", 9 => "A", 10 => "Bb", 11 => "B",
    };
}

// Lean of a scale/tonic pitch class value towards sharps/flats.
pub enum Lean {
    Sharp,
    Flat,
}

impl Lean {
    /// Get lean by considering the scale tonic pitch class value.
    ///
    /// Whether a key will be represented by sharps or flats is determined by where it falls on the
    /// circle of fifths (https://en.wikipedia.org/wiki/Circle_of_fifths). By iterating the scale
    /// tonic value by a step of -7 (in semitones) until it reaches 0 we find how much the scale
    /// leans towards being a sharp key, and likewise with a step of 7 for how much a flat key.
    /// Whichever case reaches 0 first determines the lean.
    pub fn from_tonic(value: u16) -> Self {
        let mut s = value as i16;
        let mut f = value as i16;
        loop {
            if s == 0 {
                return Lean::Sharp;
            } else if f == 0 {
                return Lean::Flat;
            }
            s = (s - 7).rem_euclid(12);
            f = (f + 7).rem_euclid(12);
        }
    }
}

/// Parse a pitch token to its pitch class value.
fn parse_pitch(token: &str) -> Result<u16, Error> {
    let token = token.to_lowercase();
    NAME_TO_VALUE
        .get(token.as_str())
        .copied()
        .ok_or_else(|| Error::InvalidPitch(token))
}

/// Parse a pitch token as the tonic of a scale to its pitch class value and associated lean.
///
/// If the pitch token is uppercase it represents tonic of a major scale, else of a minor scale. To
/// find the lean of the scale we consider the relative major of the tonic. If already major,
/// nothing needs to be done and `Lean::from_tonic` is called with the tonic pitch class value as
/// is. If minor we first need to translate the value by three semitones to get the relative major
/// of the minor scale tonic, then `Lean::from_tonic` is called.
fn parse_tonic(token: &str) -> Result<(u16, Lean), Error> {
    let value = parse_pitch(token)?;
    let is_major = token.starts_with(|c: char| ('A'..='Z').contains(&c));
    let lean = if is_major {
        Lean::from_tonic(value)
    } else {
        Lean::from_tonic((value + 3) % 12)
    };

    Ok((value, lean))
}

/// Parse a intervals token string as inter-note intervals.
fn parse_intervals(tokens: &str) -> Result<Vec<u16>, Error> {
    let intervals: Vec<u16> = tokens
        .chars()
        .filter_map(|c| match c {
            'm' => Some(1),
            'M' => Some(2),
            'A' => Some(3),
            _ => None,
        })
        .collect();

    (intervals.len() == tokens.len())
        .then_some(intervals)
        .ok_or_else(|| Error::InvalidIntervals(tokens.into()))
}

/// Scale abstraction.
///
/// The `pitches` field contains as a `u16` bitmask the information of whether a pitch is in the
/// scale, where the bits from right to left represents the pitches C, C sharp/D flat, D, etc. The
/// `tonic` field represents the pitch class value of the scale's tonic (or key center). Lastly the
/// `lean` field represents whether the pitch names of the scale are to be represented as sharps or
/// as flats.
pub struct Scale {
    pub tonic: u16,
    pub pitches: u16,
    pub lean: Lean,
}

impl Scale {
    /// New scale from a tonic pitch name token and interval tokens.
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let (tonic, lean) = parse_tonic(tonic)?;
        let intervals = parse_intervals(intervals)?;
        // First we convert the inter-value intervals to intervals from tonic by taking the modular
        // cumulative sum of the intervals (and by starting the accumulation with the tonic pitch
        // class value we also accumulate the pitch class values for the scale for free). Then we
        // fold these values into the mask.
        let pitches = intervals
            .iter()
            .accumulate(tonic, |a, b| (a + b) % 12)
            .fold(1 << tonic, |pitches, i| pitches | (1 << i));

        Ok(Self {
            tonic,
            pitches,
            lean,
        })
    }

    /// New chromatic scale, starting from a tonic pitch name.
    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmm")
    }

    /// If the scale contains a specified pitch class value.
    pub fn contains_pitch(&self, value: u16) -> bool {
        self.pitches & (1 << value) > 0
    }

    /// Enumerate the scale as pitch names (sharp/flat determined by lean of the scale).
    pub fn enumerate(&self) -> Vec<String> {
        let Self { tonic, lean, .. } = self;

        let pitch_name = match lean {
            Lean::Sharp => |p: &u16| VALUE_TO_SHARP_NAMES[p],
            Lean::Flat => |p: &u16| VALUE_TO_FLAT_NAMES[p],
        };

        (0..12)
            .filter_map(|value| {
                let value = (value + tonic) % 12;
                (self.contains_pitch(value)).then_some(pitch_name(&value).to_owned())
            })
            .collect()
    }
}
