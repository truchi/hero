use crate::Rgb;
use music::*;
// ├
// ┤

//                                     ┌───┐
// ┌───┐ ┌───┐       ┌───┐ ┌───┐ ┌───┐ │##3│ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐
// │ #7│ │##7│       │ #2│ │##2│ │ #3│ └───┘ │##4│ │ #5│ │##5│ │ #6│ │##6│
// └───┘ └───┘       └───┘ └───┘ └───┘ ┌───┐ └───┘ └───┘ └───┘ └───┘ └───┘
// ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ │ #4│ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐
// │  1│ │ b2│ │  2│ │ b3│ │  3│ │  4│ └───┘ │  5│ │ b6│ │  6│ │ b7│ │  7│
// └───┘ └───┘ └───┘ └───┘ └───┘ └───┘ ┌───┐ └───┘ └───┘ └───┘ └───┘ └───┘
// ┌───┐       ┌───┐ ┌───┐ ┌───┐ ┌───┐ │ b5│ ┌───┐       ┌───┐
// │bb2│       │bb3│ │bb4│ │ b4│ │bb5│ └───┘ │bb6│       │bb7│
// └───┘       └───┘ └───┘ └───┘ └───┘ ┌───┐ └───┘       └───┘
//                                     │  4│
//                                     └───┘

#[derive(Copy, Clone, Debug)]
pub struct Palette {
    pub background: Rgb,

    pub double_flat_unisson: Rgb,
    pub flat_unisson: Rgb,
    pub natural_unisson: Rgb,
    pub sharp_unisson: Rgb,
    pub double_sharp_unisson: Rgb,

    pub double_flat_second: Rgb,
    pub flat_second: Rgb,
    pub natural_second: Rgb,
    pub sharp_second: Rgb,
    pub double_sharp_second: Rgb,

    pub double_flat_third: Rgb,
    pub flat_third: Rgb,
    pub natural_third: Rgb,
    pub sharp_third: Rgb,
    pub double_sharp_third: Rgb,

    pub double_flat_fourth: Rgb,
    pub flat_fourth: Rgb,
    pub natural_fourth: Rgb,
    pub sharp_fourth: Rgb,
    pub double_sharp_fourth: Rgb,

    pub double_flat_fifth: Rgb,
    pub flat_fifth: Rgb,
    pub natural_fifth: Rgb,
    pub sharp_fifth: Rgb,
    pub double_sharp_fifth: Rgb,

    pub double_flat_sixth: Rgb,
    pub flat_sixth: Rgb,
    pub natural_sixth: Rgb,
    pub sharp_sixth: Rgb,
    pub double_sharp_sixth: Rgb,

    pub double_flat_seventh: Rgb,
    pub flat_seventh: Rgb,
    pub natural_seventh: Rgb,
    pub sharp_seventh: Rgb,
    pub double_sharp_seventh: Rgb,
}

impl Palette {
    pub fn rgb(&self, interval: Interval) -> Rgb {
        match interval {
            Interval(DoubleFlat, Unisson) => self.double_flat_unisson,
            Interval(Flat, Unisson) => self.flat_unisson,
            Interval(Natural, Unisson) => self.natural_unisson,
            Interval(Sharp, Unisson) => self.sharp_unisson,
            Interval(DoubleSharp, Unisson) => self.double_sharp_unisson,

            Interval(DoubleFlat, Second) => self.double_flat_second,
            Interval(Flat, Second) => self.flat_second,
            Interval(Natural, Second) => self.natural_second,
            Interval(Sharp, Second) => self.sharp_second,
            Interval(DoubleSharp, Second) => self.double_sharp_second,

            Interval(DoubleFlat, Third) => self.double_flat_third,
            Interval(Flat, Third) => self.flat_third,
            Interval(Natural, Third) => self.natural_third,
            Interval(Sharp, Third) => self.sharp_third,
            Interval(DoubleSharp, Third) => self.double_sharp_third,

            Interval(DoubleFlat, Fourth) => self.double_flat_fourth,
            Interval(Flat, Fourth) => self.flat_fourth,
            Interval(Natural, Fourth) => self.natural_fourth,
            Interval(Sharp, Fourth) => self.sharp_fourth,
            Interval(DoubleSharp, Fourth) => self.double_sharp_fourth,

            Interval(DoubleFlat, Fifth) => self.double_flat_fifth,
            Interval(Flat, Fifth) => self.flat_fifth,
            Interval(Natural, Fifth) => self.natural_fifth,
            Interval(Sharp, Fifth) => self.sharp_fifth,
            Interval(DoubleSharp, Fifth) => self.double_sharp_fifth,

            Interval(DoubleFlat, Sixth) => self.double_flat_sixth,
            Interval(Flat, Sixth) => self.flat_sixth,
            Interval(Natural, Sixth) => self.natural_sixth,
            Interval(Sharp, Sixth) => self.sharp_sixth,
            Interval(DoubleSharp, Sixth) => self.double_sharp_sixth,

            Interval(DoubleFlat, Seventh) => self.double_flat_seventh,
            Interval(Flat, Seventh) => self.flat_seventh,
            Interval(Natural, Seventh) => self.natural_seventh,
            Interval(Sharp, Seventh) => self.sharp_seventh,
            Interval(DoubleSharp, Seventh) => self.double_sharp_seventh,
        }
    }
}
