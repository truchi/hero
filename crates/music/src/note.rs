use crate::*;

// ********************************************************************************************** //
// * NoteName                                                                                   * //
// ********************************************************************************************** //

pub use NoteName::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl NoteName {
    pub fn double_flat(&self) -> Note {
        Note(*self, DoubleFlat)
    }

    pub fn flat(&self) -> Note {
        Note(*self, Flat)
    }

    pub fn natural(&self) -> Note {
        Note(*self, Natural)
    }

    pub fn sharp(&self) -> Note {
        Note(*self, Sharp)
    }

    pub fn double_sharp(&self) -> Note {
        Note(*self, DoubleSharp)
    }

    pub fn semitones(&self) -> u8 {
        match self {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        }
    }
}

// ********************************************************************************************** //
// * Note                                                                                       * //
// ********************************************************************************************** //

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Note(pub NoteName, pub Accidental);

impl Note {
    pub const fn name(&self) -> NoteName {
        self.0
    }

    pub const fn accidental(&self) -> Accidental {
        self.1
    }

    pub fn semitones(&self) -> u8 {
        const OCTAVE: i8 = crate::OCTAVE as i8;

        let semitones = self.name().semitones() as i8 + self.accidental().semitones();

        (((semitones % OCTAVE) + OCTAVE) % OCTAVE) as u8
    }
}

impl std::fmt::Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.name(), self.accidental())
    }
}
