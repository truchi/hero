use crate::palette::Palette;
use crate::Ctx;
use music::*;

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub open_width: f64,
    pub box_width: f64,
    pub box_height: f64,
    pub box_border_width: f64,
    pub box_horizontal_margin: f64,
    pub box_vertical_margin: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum Tuning {
    Four([Note; 4]),
    Five([Note; 5]),
    Six([Note; 6]),
    Seventh([Note; 7]),
    Eight([Note; 8]),
}

#[derive(Copy, Clone, Debug)]
pub struct Guitar {
    pub style: Style,
    pub frets: u8,
    pub tuning: Tuning,
    pub palette: Palette,
}

impl Guitar {
    pub fn render(&self, ctx: &Ctx, (x, y): (f64, f64), mode: Mode) {}
}

#[derive(Copy, Clone, Debug)]
struct String<'a> {
    guitar: &'a Guitar,
}

impl<'a> String<'a> {
    pub fn render(&self, ctx: &Ctx, (x, y): (f64, f64), mode: Mode) {}
}
