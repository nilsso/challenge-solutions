use std::convert::{TryFrom, TryInto};
use std::iter::{once, repeat};
use std::ops::Add;

mod accumulate;
use accumulate::Accumulate;

#[derive(Debug)]
pub enum Error {
    InvalidPitchValue(u8),
    InvalidPitchName(String),
    InvalidIntervals(String),
}

#[derive(Copy, Clone)]
pub enum PitchName {
    Single(&'static str),
    Diatonic(&'static str, &'static str),
}

impl PitchName {
    pub fn sharp_name(&self) -> &str {
        match self {
            PitchName::Single(n) => n,
            PitchName::Diatonic(n, _) => n,
        }
    }

    pub fn flat_name(&self) -> &str {
        match self {
            PitchName::Single(n) => n,
            PitchName::Diatonic(_, n) => n,
        }
    }
}

macro_rules! pitch_name {
    ($name:expr) => {
        PitchName::Single($name)
    };
    ($sharp_name:expr, $flat_name:expr) => {
        PitchName::Diatonic($sharp_name, $flat_name)
    };
}

#[derive(Copy, Clone)]
pub struct Pitch {
    pub value: u8,
    pub name: PitchName,
}

impl TryFrom<u8> for Pitch {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        #[rustfmt::skip]
        let name = match value % 12 {
            0  => pitch_name!("C"),
            1  => pitch_name!("C#", "Db"),
            2  => pitch_name!("D"),
            3  => pitch_name!("D#", "Eb"),
            4  => pitch_name!("E"),
            5  => pitch_name!("F"),
            6  => pitch_name!("F#", "Gb"),
            7  => pitch_name!("G"),
            8  => pitch_name!("G#", "Ab"),
            9  => pitch_name!("A"),
            10 => pitch_name!("A#", "Bb"),
            11 => pitch_name!("B"),
            _ => return Err(Error::InvalidPitchValue(value)),
        };

        Ok(Self { value, name })
    }
}

enum Lean {
    Sharp,
    Flat,
}

impl Lean {
    /// Determines the lean for a given major key.
    ///
    /// Using the pitch name of the major key tonal center, the easiest way to determine the lean
    /// is to count the number of steps around the circle of fifths it takes to get to C (i.e. 0),
    /// clockwise (plus 7 semitones) or counter-clockwise (minus  7 semitones). If clockwise is
    /// smaller, the key has a flat accidental lean. Else if counter-clockwise is smaller, the key
    /// has a sharp accidental lean.
    pub fn from_major(value: u8) -> Lean {
        let steps_to_cycle = |step: i8| -> u8 {
            let mut n = value as i8;
            let mut i = 0;
            while n > 0 {
                i += 1;
                n = (n - step).rem_euclid(12);
            }
            i
        };

        let from_sharps = steps_to_cycle(7);
        let from_flats = steps_to_cycle(-7);

        if from_sharps <= from_flats {
            Lean::Sharp
        } else {
            Lean::Flat
        }
    }
}

// Parse pitch/tonal center name to pitch value
// (Lowercase to match minor keys)
fn parse_name(token: &str) -> Result<u8, Error> {
    #[rustfmt::skip]
    let value = match token.to_lowercase().as_str() {
        "c"  => 0,
        "c#" => 1,  "db" => 1,
        "d"  => 2,
        "d#" => 3,  "eb" => 3,
        "e"  => 4,
        "f"  => 5,
        "f#" => 6,  "gb" => 6,
        "g"  => 7,
        "g#" => 8,  "ab" => 8,
        "a"  => 9,
        "a#" => 10, "bb" => 10,
        "b"  => 11,
        _ => return Err(Error::InvalidPitchName(token.into())),
    };

    Ok(value)
}

// Parse inter-pitches intervals.
fn parse_intervals(tokens: &str) -> Result<Vec<u8>, Error> {
    let mut res = vec![];
    for c in tokens.chars() {
        match c {
            'm' => res.push(1),
            'M' => res.push(2),
            'A' => res.push(3),
            _ => return Err(Error::InvalidIntervals(tokens.into())),
        }
    }
    Ok(res)
}

/// Parse the scale tonal center.
///
/// Returns the pitch value of the tonal center and the accidental lean.
///
/// The pitch value is the same regardless of major or minor (e.g. f# == 6 == F#).
/// However, to determine the lean we need to translate the given key to its relative major.
/// If the key is already major, then we're done. If the key is minor, then we add three semitones
/// to the pitch value (e.g. f# = 6 -> A = 9). The lean is then determined from the relative major.
fn parse_tonic(token: &str) -> Result<(u8, Lean), Error> {
    let c = token.chars().next().unwrap();
    let value = parse_name(token)?;

    if ('A'..='G').contains(&c) {
        Ok((value, Lean::from_major(value)))
    } else {
        let major = (value + 3) % 12;
        Ok((value, Lean::from_major(major)))
    }
}

pub struct Scale {
    lean: Lean,
    pitches: Vec<Pitch>,
}

impl TryFrom<(&str, Vec<u8>)> for Scale {
    type Error = Error;

    fn try_from((tonic, intervals): (&str, Vec<u8>)) -> Result<Scale, Self::Error> {
        let (tonic_value, lean) = parse_tonic(tonic)?;

        // Take the cumulative sum (starting with the tonic value) of the inter-pitch intervals to
        // get intervals above the tonic value.
        let accumlating_intervals = intervals.iter().accumulate(tonic_value, <u8>::add);

        let mut pitches = vec![];

        // Then it's easier to just chain the tonic value once with the accumulated intervals, and
        // only take as many values as there were intervals. The way the intervals are translated
        // into a scale in this exercise is strange to me. If I see the list of inter-pitch
        // intervals 'MMmMMMm' (as in ionian major) starting above the tonal center C I would
        // expect to produce the notes:
        //
        // C-D-E-F-G-A-B-C, an 8 note scale
        //
        // But instead we ignore the last interval.
        for value in once(tonic_value)
            .chain(accumlating_intervals)
            .take(intervals.len())
        {
            pitches.push(value.try_into()?);
        }

        Ok(Self { lean, pitches })
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let intervals = parse_intervals(intervals)?;
        Ok((tonic, intervals).try_into()?)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let intervals = repeat(1).take(12).collect();
        Ok((tonic, intervals).try_into()?)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let Self { lean, pitches } = self;

        let f = match lean {
            Lean::Sharp => |p: &Pitch| p.name.sharp_name().to_owned(),
            Lean::Flat => |p: &Pitch| p.name.flat_name().to_owned(),
        };

        pitches.iter().map(f).collect()
    }
}
