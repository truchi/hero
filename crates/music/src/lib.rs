pub mod accidental;
pub mod dictionary;
pub mod interval;
pub mod list;
pub mod note;

pub use accidental::*;
pub use interval::*;
pub use note::*;

pub const OCTAVE: u8 = 12;

// ======================

use list::*;

#[derive(Debug)]
pub struct Mode(Note, Intervals);

impl Mode {
    pub fn root(&self) -> Note {
        self.0
    }

    pub fn intervals(&self) -> Intervals {
        self.1
    }
}
