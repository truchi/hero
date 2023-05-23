use crate::*;

scales! {
    //
    // Five notes
    //
    pentatonic("Pentatonic", 5, 5): {
        major("Major"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Natural.fifth(),
            Natural.sixth(),
        ],
        egyptian("Egyptian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .seventh(),
        ],
        malkos_raga("Malkos raga"): intervals![
            Natural.unisson(),
            Flat   .third(),
            Natural.fourth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        ritusen("Ritusen"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
        ],
        minor("Minor"): intervals![
            Natural.unisson(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .seventh(),
        ],
    },
    //
    // Six notes
    //
    augmented("Augmented", 6, 2): {
        augmented("Augmented"): intervals![
            Natural.unisson(),
            Sharp  .second(),
            Natural.third(),
            Natural.fifth(),
            Sharp .fifth(),
            Natural.seventh(),
        ],
        six_tone_symmetric("Six tone symmetric"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Natural.third(),
            Natural.fourth(),
            Sharp .fifth(),
            Natural.sixth(),
        ],
    },
    whote_tone("Whole tone", 6, 1): {
        augmented("Augmented"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Flat   .fifth(),
            Sharp  .fifth(),
            Flat   .seventh(),
        ],
    },
    //
    // Seven notes
    //
    diatonic("Diatonic", 7, 7): {
        ionian("Ionian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        dorian("Dorian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        phrygian("Phrygian"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        lydian("Lydian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Sharp  .fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        mixolydian("Mixolydian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        aeolian("Aeolian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        locrian("Locrian"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Flat   .third(),
            Natural.fourth(),
            Flat   .fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
    },
    harmonic("Harmonic", 7, 7): {
        aeolian_sharp_seventh("Aeolian ♯7"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .sixth(),
            Natural.seventh(),
        ],
        locrian_sharp_sixth("Locrian ♯6"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Flat   .third(),
            Natural.fourth(),
            Flat   .fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        ionian_sharp_fifth("Ionian ♯5"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Natural.fourth(),
            Sharp  .fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        dorian_sharp_fourth("Dorian ♯4"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Sharp  .fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        phrygian_sharp_third("Phrygian ♯3"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Natural.third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        lydian_sharp_second("Lydian ♯2"): intervals![
            Natural.unisson(),
            Sharp  .second(),
            Natural.third(),
            Sharp  .fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        superlocrian_double_flat_seventh("Superlocrian ♭♭7"): intervals![
            Natural   .unisson(),
            Flat      .second(),
            Flat      .third(),
            Flat      .fourth(),
            Flat      .fifth(),
            Flat      .sixth(),
            DoubleFlat.seventh(),
        ],
    },
    melodic("Melodic", 7, 7): {
        aeolian("Aeolian"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        dorian_flat_second("Dorian ♭2"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Flat   .third(),
            Natural.fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        lydian_augmented("Lydian ♯5"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Sharp  .fourth(),
            Sharp  .fifth(),
            Natural.sixth(),
            Natural.seventh(),
        ],
        lydian_dominant("Lydian ♭7"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Sharp  .fourth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
        mixolydian_flat_sixth("Mixolydian ♭6"): intervals![
            Natural.unisson(),
            Natural.second(),
            Natural.third(),
            Natural.fourth(),
            Natural.fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        half_diminished("Half diminished"): intervals![
            Natural.unisson(),
            Natural.second(),
            Flat   .third(),
            Natural.fourth(),
            Flat   .fifth(),
            Flat   .sixth(),
            Flat   .seventh(),
        ],
        altered("Altered"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Sharp  .second(),
            Natural.third(),
            Flat   .fifth(),
            Sharp  .fifth(),
            Flat   .seventh(),
        ],
    },
    //
    // Eight notes
    //
    diminished("Diminished", 7, 7): {
        whole_half("Whole half"): intervals![
            Natural   .unisson(),
            Natural   .second(),
            Flat      .third(),
            Natural   .fourth(),
            Flat      .fifth(),
            Sharp     .fifth(),
            DoubleFlat.seventh(),
            Natural   .seventh(),
        ],
        half_whole("Half whole"): intervals![
            Natural.unisson(),
            Flat   .second(),
            Sharp  .second(),
            Natural.third(),
            Flat   .fifth(),
            Natural.fifth(),
            Natural.sixth(),
            Flat   .seventh(),
        ],
    },
}

pub struct Dictionary {}

#[derive(Copy, Clone, Debug)]
pub struct DictionaryScale {
    name: &'static str,
    modes: List<DictionaryMode>,
    notes: u8,
    symmetry: u8,
}

impl DictionaryScale {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn modes(&self) -> List<DictionaryMode> {
        self.modes
    }

    pub const fn notes(&self) -> u8 {
        self.notes
    }

    pub const fn symmetry(&self) -> u8 {
        self.symmetry
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DictionaryMode {
    name: &'static str,
    intervals: Intervals,
    notes: u8,
    symmetry: u8,
}

impl DictionaryMode {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intervals(&self) -> Intervals {
        self.intervals
    }

    pub const fn notes(&self) -> u8 {
        self.notes
    }

    pub const fn symmetry(&self) -> u8 {
        self.symmetry
    }
}

macro_rules! scales {
    ( $(
        $scale:ident($scale_name:literal, $notes:literal, $symmetry:literal): { $(
            $mode:ident($mode_name:literal): $intervals:expr,
        )* },
    )* ) => {
        $(
            #[doc = $scale_name]
            pub fn $scale() -> DictionaryScale {
                DictionaryScale {
                    name: $scale_name,
                    modes: modes!($($scale::$mode(),)*),
                    notes: $notes,
                    symmetry: $symmetry,
                }
            }

            #[doc = $scale_name]
            pub mod $scale {
                use super::*;

                $(
                    #[doc = $mode_name]
                    pub const fn $mode() -> DictionaryMode {
                        DictionaryMode {
                            name: $mode_name,
                            intervals: $intervals,
                            notes: $notes,
                            symmetry: $symmetry,
                        }
                    }
                )*
            }
        )*

        // #[cfg(test)]
        // mod tests {
        //     $(
        //         #[test]
        //         fn $scale() {
        //             super::test_utils::test(super::$scale());
        //         }
        //     )*
        // }
    }
}

use scales;

macro_rules! modes {
    () => {
        crate::List::from_raw(
            0,
            [
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr $(,)?) => {
        crate::List::from_raw(
            1,
            [
                $a,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr $(,)?) => {
        crate::List::from_raw(
            2,
            [
                $a,
                $b,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        crate::List::from_raw(
            3,
            [
                $a,
                $b,
                $c,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        crate::List::from_raw(
            4,
            [
                $a,
                $b,
                $c,
                $d,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        crate::List::from_raw(
            5,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $(,)?) => {
        crate::List::from_raw(
            6,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $(,)?) => {
        crate::List::from_raw(
            7,
            [
                $a,
                $b,
                $c,
                $d,
                $e,
                $f,
                $g,
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr $(,)?) => {
        crate::List::from_raw(
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
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr $(,)?) => {
        crate::List::from_raw(
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
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr $(,)?) => {
        crate::List::from_raw(
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
                whatever_dictonary_mode(),
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr $(,)?) => {
        crate::List::from_raw(
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
                whatever_dictonary_mode(),
            ],
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr $(,)?) => {
        crate::List::from_raw(12, [$a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l])
    };
}

use modes;

const fn whatever_dictonary_mode() -> DictionaryMode {
    DictionaryMode {
        name: "",
        intervals: intervals!(),
        notes: 0,
        symmetry: 0,
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn _test(modes: List<Intervals>) {
        let mut modes = modes.as_ref().into_iter().copied().map(|i| i.0);

        if let Some(first) = modes.next() {
            let mut first = _relative(first);

            for mode in modes {
                let removed = first.remove(0);
                first.push(removed);

                assert_eq!(first, _relative(mode));
            }
        }
    }

    pub fn _relative(mode: List<Interval>) -> Vec<u8> {
        let mut intervals = mode.as_ref().into_iter();
        let mut relative = vec![];

        if let Some(mut prev) = intervals.next() {
            for interval in intervals {
                relative.push(interval.semitones() - prev.semitones());
                prev = interval;
            }

            relative.push(OCTAVE - prev.semitones());
        }

        relative
    }
}
