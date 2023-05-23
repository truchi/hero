use crate::*;

// ********************************************************************************************** //
// * IntervalName                                                                               * //
// ********************************************************************************************** //

pub use IntervalName::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum IntervalName {
    Unisson,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl IntervalName {
    pub fn semitones(&self) -> u8 {
        match self {
            Unisson => 0,
            Second => 2,
            Third => 4,
            Fourth => 5,
            Fifth => 7,
            Sixth => 9,
            Seventh => 11,
        }
    }
}

impl std::fmt::Debug for IntervalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        f.write_char(match self {
            Unisson => '1',
            Second => '2',
            Third => '3',
            Fourth => '4',
            Fifth => '5',
            Sixth => '6',
            Seventh => '7',
        })
    }
}

// ********************************************************************************************** //
// * Interval                                                                                   * //
// ********************************************************************************************** //

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interval(pub Accidental, pub IntervalName);

impl Interval {
    pub const fn accidental(&self) -> Accidental {
        self.0
    }

    pub const fn name(&self) -> IntervalName {
        self.1
    }

    pub fn semitones(&self) -> u8 {
        const OCTAVE: i8 = crate::OCTAVE as i8;

        let semitones = self.name().semitones() as i8 + self.accidental().semitones();

        (((semitones % OCTAVE) + OCTAVE) % OCTAVE) as u8
    }
}

impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.accidental(), self.name())
    }
}

// ********************************************************************************************** //
// * List<Interval>                                                                             * //
// ********************************************************************************************** //

const _CHROMATIC: [Interval; 12] = [
    Interval(Natural, Unisson),
    Interval(Flat, Second),
    Interval(Natural, Second),
    Interval(Flat, Third),
    Interval(Natural, Third),
    Interval(Natural, Fourth),
    Interval(Flat, Fifth),
    Interval(Natural, Fifth),
    Interval(Flat, Sixth),
    Interval(Natural, Sixth),
    Interval(Flat, Seventh),
    Interval(Natural, Seventh),
];

const _DIATONIC: [Interval; 7] = [
    Interval(Natural, Unisson),
    Interval(Natural, Second),
    Interval(Natural, Third),
    Interval(Natural, Fourth),
    Interval(Natural, Fifth),
    Interval(Natural, Sixth),
    Interval(Natural, Seventh),
];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Intervals(pub List<Interval>);

impl Intervals {
    pub const CHROMATIC: Self = intervals![
        Interval(Natural, Unisson),
        Interval(Flat, Second),
        Interval(Natural, Second),
        Interval(Flat, Third),
        Interval(Natural, Third),
        Interval(Natural, Fourth),
        Interval(Flat, Fifth),
        Interval(Natural, Fifth),
        Interval(Flat, Sixth),
        Interval(Natural, Sixth),
        Interval(Flat, Seventh),
        Interval(Natural, Seventh),
    ];
}

impl FromIterator<Interval> for Intervals {
    fn from_iter<I: IntoIterator<Item = Interval>>(items: I) -> Self {
        Self(List::from_iter(items))
    }
}

impl<const N: usize> From<[Interval; N]> for Intervals {
    fn from(items: [Interval; N]) -> Self {
        Self(List::from(items))
    }
}

#[macro_export]
macro_rules! intervals {
    () => {
        $crate::Intervals($crate::list::List::from_raw(
            0,
            [
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            1,
            [
                $a,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            2,
            [
                $a,
                $b,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            3,
            [
                $a,
                $b,
                $c,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            4,
            [
                $a,
                $b,
                $c,
                $d,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            5,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            6,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            7,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            8,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                $h,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            9,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                $h,
                $i,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            10,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                $h,
                $i,
                $j,
                $crate::Natural.unisson(),
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            11,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                $h,
                $i,
                $j,
                $k,
                $crate::Natural.unisson(),
            ],
        ))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr $(,)?) => {
        $crate::Intervals($crate::list::List::from_raw(
            12,
            [$a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l],
        ))
    };
}
pub use intervals;
