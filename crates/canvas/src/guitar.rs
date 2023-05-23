use super::*;
use crate::palette::Palette;
use music::*;

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub open_width: f64,
    pub box_width: f64,
    pub box_height: f64,
    pub box_radius: f64,
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

impl AsRef<[Note]> for Tuning {
    fn as_ref(&self) -> &[Note] {
        match self {
            Tuning::Four(notes) => &notes[..],
            Tuning::Five(notes) => &notes[..],
            Tuning::Six(notes) => &notes[..],
            Tuning::Seventh(notes) => &notes[..],
            Tuning::Eight(notes) => &notes[..],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Guitar {
    pub style: Style,
    pub frets: u8,
    pub tuning: Tuning,
}

impl Guitar {
    pub fn render<'a, T: Fn(Interval) -> &'a Color>(
        &self,
        ctx: &Ctx,
        (x, y): (f64, f64),
        mode: Mode,
        palette: T,
        clip: Option<Triangle>,
        fill_or_stroke: FillOrStroke,
    ) {
        for (i, note) in self.tuning.as_ref().iter().copied().rev().enumerate() {
            let y = y + i as f64 * (self.style.box_height + self.style.box_vertical_margin);
            String(note).render(ctx, (x, y), mode, &palette, self, clip, fill_or_stroke);
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct String(Note);

impl String {
    pub fn render<'a, T: Fn(Interval) -> &'a Color>(
        &self,
        ctx: &Ctx,
        (x, y): (f64, f64),
        mode: Mode,
        palette: T,
        guitar: &Guitar,
        clip: Option<Triangle>,
        fill_or_stroke: FillOrStroke,
    ) {
        let semitones = self.0.semitones();

        (0..=guitar.frets)
            .flat_map(|fret| {
                let x = x + if fret == 0 {
                    0.
                } else {
                    let open = guitar.style.open_width;
                    let margin = guitar.style.box_horizontal_margin;
                    let width = guitar.style.box_width;

                    open + margin + (fret - 1) as f64 * (width + margin)
                };
                let w = if fret == 0 {
                    guitar.style.open_width
                } else {
                    guitar.style.box_width
                };

                if let Some(interval) = mode.interval(semitones + fret) {
                    Some((
                        super::Rect {
                            x,
                            y,
                            w,
                            h: guitar.style.box_height,
                            r: guitar.style.box_radius,
                            b: guitar.style.box_border_width,
                            clip: Some(super::Triangle::BottomRight(50.)),
                        },
                        palette(interval),
                    ))
                } else {
                    None
                }
            })
            .for_each(|(b, color)| b.render(ctx, color, clip, fill_or_stroke));
    }
}
