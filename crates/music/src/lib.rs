pub mod accidental;
pub mod dictionary;
pub mod interval;
pub mod list;
pub mod note;
pub mod rythm;
pub mod score;

pub use accidental::*;
pub use interval::*;
pub use note::*;
pub use rythm::*;
pub use score::*;

pub const OCTAVE: u8 = 12;

// ======================

use list::*;

#[derive(Copy, Clone, Debug)]
pub struct Mode(pub Note, pub Intervals);

impl Mode {
    pub fn root(&self) -> Note {
        self.0
    }

    pub fn intervals(&self) -> Intervals {
        self.1
    }

    pub fn interval(&self, semitones: u8) -> Option<Interval> {
        let semitones = semitones % OCTAVE;
        self.intervals()
            .0
            .as_ref()
            .iter()
            .find(|interval| interval.semitones() == semitones)
            .copied()
    }
}
