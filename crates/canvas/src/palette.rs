use crate::Color;
use music::*;
use wasm_bindgen::JsValue;

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

pub fn hsl(h: u16, s: u8, l: u8) -> Color {
    Color::from_str(&format!("hsl({} {}% {}%)", h, s, l))
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub background: Color,

    pub double_flat_unisson: Color,
    pub flat_unisson: Color,
    pub natural_unisson: Color,
    pub sharp_unisson: Color,
    pub double_sharp_unisson: Color,

    pub double_flat_second: Color,
    pub flat_second: Color,
    pub natural_second: Color,
    pub sharp_second: Color,
    pub double_sharp_second: Color,

    pub double_flat_third: Color,
    pub flat_third: Color,
    pub natural_third: Color,
    pub sharp_third: Color,
    pub double_sharp_third: Color,

    pub double_flat_fourth: Color,
    pub flat_fourth: Color,
    pub natural_fourth: Color,
    pub sharp_fourth: Color,
    pub double_sharp_fourth: Color,

    pub double_flat_fifth: Color,
    pub flat_fifth: Color,
    pub natural_fifth: Color,
    pub sharp_fifth: Color,
    pub double_sharp_fifth: Color,

    pub double_flat_sixth: Color,
    pub flat_sixth: Color,
    pub natural_sixth: Color,
    pub sharp_sixth: Color,
    pub double_sharp_sixth: Color,

    pub double_flat_seventh: Color,
    pub flat_seventh: Color,
    pub natural_seventh: Color,
    pub sharp_seventh: Color,
    pub double_sharp_seventh: Color,
}

impl Palette {
    pub fn background(&self) -> &Color {
        &self.background
    }

    pub fn color(&self, interval: Interval) -> &Color {
        let Interval(accidental, name) = interval;

        match accidental {
            DoubleFlat => match name {
                Unisson => &self.double_flat_unisson,
                Second => &self.double_flat_second,
                Third => &self.double_flat_third,
                Fourth => &self.double_flat_fourth,
                Fifth => &self.double_flat_fifth,
                Sixth => &self.double_flat_sixth,
                Seventh => &self.double_flat_seventh,
            },
            Flat => match name {
                Unisson => &self.flat_unisson,
                Second => &self.flat_second,
                Third => &self.flat_third,
                Fourth => &self.flat_fourth,
                Fifth => &self.flat_fifth,
                Sixth => &self.flat_sixth,
                Seventh => &self.flat_seventh,
            },
            Natural => match name {
                Unisson => &self.natural_unisson,
                Second => &self.natural_second,
                Third => &self.natural_third,
                Fourth => &self.natural_fourth,
                Fifth => &self.natural_fifth,
                Sixth => &self.natural_sixth,
                Seventh => &self.natural_seventh,
            },
            Sharp => match name {
                Unisson => &self.sharp_unisson,
                Second => &self.sharp_second,
                Third => &self.sharp_third,
                Fourth => &self.sharp_fourth,
                Fifth => &self.sharp_fifth,
                Sixth => &self.sharp_sixth,
                Seventh => &self.sharp_seventh,
            },
            DoubleSharp => match name {
                Unisson => &self.double_sharp_unisson,
                Second => &self.double_sharp_second,
                Third => &self.double_sharp_third,
                Fourth => &self.double_sharp_fourth,
                Fifth => &self.double_sharp_fifth,
                Sixth => &self.double_sharp_sixth,
                Seventh => &self.double_sharp_seventh,
            },
        }
    }
}
