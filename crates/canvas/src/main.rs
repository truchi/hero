#![allow(unused)]

pub mod guitar;
pub mod palette;

use guitar::Guitar;
use guitar::Style;
use music::*;
use palette::Palette;
use std::f64;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d as Ctx;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused)]
macro_rules! log {
    ($($t:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe { log(&format_args!($($t)*).to_string()) }
    };
}

fn main() {
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.style()
        .set_property("background", "rgb(10, 10, 10)")
        .unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<Ctx>()
        .unwrap();

    let palette = Palette {
        background: Rgb(0, 0, 0),
        double_flat_unisson: Rgb(255, 255, 255),
        flat_unisson: Rgb(255, 255, 255),
        natural_unisson: Rgb(255, 255, 255),
        sharp_unisson: Rgb(255, 255, 255),
        double_sharp_unisson: Rgb(255, 255, 255),
        double_flat_second: Rgb(255, 0, 0),
        flat_second: Rgb(255, 0, 0),
        natural_second: Rgb(255, 0, 0),
        sharp_second: Rgb(255, 0, 0),
        double_sharp_second: Rgb(255, 0, 0),
        double_flat_third: Rgb(0, 255, 255),
        flat_third: Rgb(0, 255, 255),
        natural_third: Rgb(0, 255, 255),
        sharp_third: Rgb(0, 255, 255),
        double_sharp_third: Rgb(0, 255, 255),
        double_flat_fourth: Rgb(0, 0, 0),
        flat_fourth: Rgb(0, 0, 0),
        natural_fourth: Rgb(0, 0, 0),
        sharp_fourth: Rgb(0, 0, 0),
        double_sharp_fourth: Rgb(0, 0, 0),
        double_flat_fifth: Rgb(0, 0, 0),
        flat_fifth: Rgb(0, 0, 0),
        natural_fifth: Rgb(0, 0, 0),
        sharp_fifth: Rgb(0, 0, 0),
        double_sharp_fifth: Rgb(0, 0, 0),
        double_flat_sixth: Rgb(0, 0, 0),
        flat_sixth: Rgb(0, 0, 0),
        natural_sixth: Rgb(0, 0, 0),
        sharp_sixth: Rgb(0, 0, 0),
        double_sharp_sixth: Rgb(0, 0, 0),
        double_flat_seventh: Rgb(0, 0, 0),
        flat_seventh: Rgb(0, 0, 0),
        natural_seventh: Rgb(0, 0, 0),
        sharp_seventh: Rgb(0, 0, 0),
        double_sharp_seventh: Rgb(0, 0, 0),
    };

    let style = Style {
        open_width: 5.,
        box_width: 15.,
        box_height: 10.,
        box_border_width: 4.,
        box_horizontal_margin: 5.,
        box_vertical_margin: 5.,
    };

    let guitar = Guitar {
        style,
        frets: 13,
        tuning: guitar::Tuning::Six([
            E.natural(),
            A.natural(),
            D.natural(),
            G.natural(),
            B.natural(),
            E.natural(),
        ]),
        palette,
    };

    log!("{guitar:#?}");

    Box {
        x: 10.,
        y: 10.,
        w: 100.,
        h: 100.,
        r: 10.,
        color: (255, 255, 0),
        border_color: (0, 255, 255),
        border_width: 10.,
        clip: Some(Triangle::BottomRight(50.)),
    }
    .render(&context);
}

#[derive(Copy, Clone, Debug)]
enum Triangle {
    TopLeft(f64),
    BottomRight(f64),
}

impl Triangle {
    fn path(&self, ctx: &Ctx, b: &Box) {
        match self {
            Self::TopLeft(percent) => {
                let ratio = 2. * percent / 100.;
                let (x, y, w, h) = b.bounding();

                let (w, h) = (w * ratio, h * ratio);

                ctx.move_to(x, y);
                ctx.line_to(x + w, y);
                ctx.line_to(x, y + h);
            }
            Self::BottomRight(percent) => {
                let ratio = 2. * percent / 100.;
                let (x, y, w, h) = b.bounding();
                let (x2, y2) = (x + w, y + h);
                let (w, h) = (w * ratio, h * ratio);

                ctx.move_to(x2, y2);
                ctx.line_to(x2 - w, y2);
                ctx.line_to(x2, y2 - h);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Box {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    r: f64,
    color: (u8, u8, u8),
    border_color: (u8, u8, u8),
    border_width: f64,
    clip: Option<Triangle>,
}

impl Box {
    fn bounding(&self) -> (f64, f64, f64, f64) {
        let (x, y, w, h, r) = (self.x, self.y, self.w, self.h, self.r);

        (x - r / 2., y - r / 2., w + r, h + r)
    }

    fn color(&self) -> Rgb {
        Rgb(self.color.0, self.color.1, self.color.2)
    }

    fn border_color(&self) -> Rgb {
        Rgb(
            self.border_color.0,
            self.border_color.1,
            self.border_color.2,
        )
    }

    fn render(&self, ctx: &Ctx) {
        ctx.save();

        if let Some(clip) = self.clip {
            ctx.begin_path();
            clip.path(ctx, self);
            ctx.clip();
        }

        ctx.begin_path();

        ctx.set_fill_style(&self.color().js_value());
        self.rect(ctx);
        ctx.fill();

        if self.border_width >= 1. {
            ctx.set_line_width(self.border_width);
            ctx.set_stroke_style(&self.border_color().js_value());
            self.rect(ctx);
            ctx.stroke();
        }

        ctx.restore();
    }

    fn rect(&self, ctx: &Ctx) {
        let (x, y, w, h, r) = (self.x, self.y, self.w, self.h, self.r);

        ctx.move_to(x, y + r);
        ctx.arc_to(x, y + h, x + r, y + h, r).unwrap();
        ctx.arc_to(x + w, y + h, x + w, y + h - r, r).unwrap();
        ctx.arc_to(x + w, y, x + w - r, y, r).unwrap();
        ctx.arc_to(x, y, x, y + r, r).unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    pub fn js_value(&self) -> JsValue {
        JsValue::from_str(&format!("rgb({},{},{})", self.0, self.1, self.2))
    }

    fn js_value_complicated(&self) -> JsValue {
        let mut str = [
            b'r', b'g', b'b', b'(', b'?', b'?', b'?', b',', b'?', b'?', b'?', b',', b'?', b'?',
            b'?', b')',
        ];

        let str = unsafe {
            debug_assert!(str.len() == 16);
            Self::write_u8(self.0, str.get_unchecked_mut(4..7));
            Self::write_u8(self.1, str.get_unchecked_mut(8..1));
            Self::write_u8(self.2, str.get_unchecked_mut(12..1));
            debug_assert!(std::str::from_utf8(&str[..]).is_ok());
            std::str::from_utf8_unchecked(&str[..])
        };

        JsValue::from_str(str)
    }

    fn write_u8(byte: u8, dest: &mut [u8]) {
        fn go(byte: u8, dest: &mut u8, it: impl IntoIterator<Item = (u8, u8)>) -> u8 {
            for (sub, char) in it {
                if let Some(byte) = byte.checked_sub(sub) {
                    *dest = char;
                    return byte;
                }
            }
            *dest = b'0';
            return byte;
        }

        let hundreds = [(200, b'2'), (100, b'1')];
        let tens = [
            (90, b'9'),
            (80, b'8'),
            (70, b'7'),
            (60, b'6'),
            (50, b'5'),
            (40, b'4'),
            (30, b'3'),
            (20, b'2'),
            (10_u8, b'1'),
        ];
        let units = [
            (9, b'9'),
            (8, b'8'),
            (7, b'7'),
            (6, b'6'),
            (5, b'5'),
            (4, b'4'),
            (3, b'3'),
            (2, b'2'),
            (1, b'1'),
        ];

        unsafe {
            debug_assert!(dest.len() > 2);
            go(
                go(
                    go(byte, dest.get_unchecked_mut(0), hundreds),
                    dest.get_unchecked_mut(1),
                    tens,
                ),
                dest.get_unchecked_mut(2),
                units,
            );
        }
    }
}
