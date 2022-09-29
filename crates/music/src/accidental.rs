use crate::*;

// ********************************************************************************************** //
// * Accidental                                                                                 * //
// ********************************************************************************************** //

pub use Accidental::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Default for Accidental {
    fn default() -> Self {
        Natural
    }
}

impl Accidental {
    pub const fn unisson(&self) -> Interval {
        Interval(*self, Unisson)
    }

    pub const fn second(&self) -> Interval {
        Interval(*self, Second)
    }

    pub const fn third(&self) -> Interval {
        Interval(*self, Third)
    }

    pub const fn fourth(&self) -> Interval {
        Interval(*self, Fourth)
    }

    pub const fn fifth(&self) -> Interval {
        Interval(*self, Fifth)
    }

    pub const fn sixth(&self) -> Interval {
        Interval(*self, Sixth)
    }

    pub const fn seventh(&self) -> Interval {
        Interval(*self, Seventh)
    }

    pub const fn semitones(&self) -> i8 {
        match self {
            DoubleFlat => -2,
            Flat => -1,
            Natural => 0,
            Sharp => 1,
            DoubleSharp => 2,
        }
    }
}

impl std::fmt::Debug for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        f.write_char(match self {
            DoubleFlat => '𝄫',
            Flat => '♭',
            Natural => ' ', // ♮
            Sharp => '♯',
            DoubleSharp => '𝄪',
        })
    }
}
