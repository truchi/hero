#![allow(unused)]

pub mod guitar;
pub mod palette;
pub mod rect;

use guitar::Guitar;
use guitar::Style;
use music::*;
use palette::Palette;
use rect::*;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d as Ctx;
use web_sys::EventTarget;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js_log(s: &str);
}

#[macro_export]
#[allow(unused)]
macro_rules! log {
    ($($t:tt)*) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::js_log(&format_args!($($t)*).to_string()) }
    }};
}

fn main() {
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.style()
        .set_property("background", "rgb(50, 50, 50)")
        .unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<Ctx>()
        .unwrap();

    let palette = {
        use palette::hsl;

        let second = 0;
        let third = 60;
        let fourth = 120;
        let fifth = 180;
        let sixth = 240;
        let seventh = 300;

        let double_flat = 20;
        let flat = 30;
        let natural = 40;
        let sharp = 50;
        let double_sharp = 60;

        Palette {
            background: hsl(0, 0, 0),

            double_flat_unisson: hsl(0, 0, 100),
            flat_unisson: hsl(0, 0, 100),
            natural_unisson: hsl(0, 0, 100),
            sharp_unisson: hsl(0, 0, 100),
            double_sharp_unisson: hsl(0, 0, 100),

            double_flat_second: hsl(second, double_flat, 50),
            flat_second: hsl(second, flat, 50),
            natural_second: hsl(second, natural, 50),
            sharp_second: hsl(second, sharp, 50),
            double_sharp_second: hsl(second, double_sharp, 50),

            double_flat_third: hsl(third, double_flat, 50),
            flat_third: hsl(third, flat, 50),
            natural_third: hsl(third, natural, 50),
            sharp_third: hsl(third, sharp, 50),
            double_sharp_third: hsl(third, double_sharp, 50),

            double_flat_fourth: hsl(fourth, double_flat, 50),
            flat_fourth: hsl(fourth, flat, 50),
            natural_fourth: hsl(fourth, natural, 50),
            sharp_fourth: hsl(fourth, sharp, 50),
            double_sharp_fourth: hsl(fourth, double_sharp, 50),

            double_flat_fifth: hsl(fifth, double_flat, 50),
            flat_fifth: hsl(fifth, flat, 50),
            natural_fifth: hsl(fifth, natural, 50),
            sharp_fifth: hsl(fifth, sharp, 50),
            double_sharp_fifth: hsl(fifth, double_sharp, 50),

            double_flat_sixth: hsl(sixth, double_flat, 50),
            flat_sixth: hsl(sixth, flat, 50),
            natural_sixth: hsl(sixth, natural, 50),
            sharp_sixth: hsl(sixth, sharp, 50),
            double_sharp_sixth: hsl(sixth, double_sharp, 50),

            double_flat_seventh: hsl(seventh, double_flat, 50),
            flat_seventh: hsl(seventh, flat, 50),
            natural_seventh: hsl(seventh, natural, 50),
            sharp_seventh: hsl(seventh, sharp, 50),
            double_sharp_seventh: hsl(seventh, double_sharp, 50),
        }
    };

    let style = Style {
        open_width: 15.,
        box_width: 30.,
        box_height: 20.,
        box_radius: 5.,
        box_border_width: 2.,
        box_horizontal_margin: 10.,
        box_vertical_margin: 10.,
    };

    let guitar = Guitar {
        style,
        frets: 15,
        tuning: guitar::Tuning::Six([
            E.natural(),
            A.natural(),
            D.natural(),
            G.natural(),
            B.natural(),
            E.natural(),
        ]),
    };

    log!("{guitar:#?}");

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        let (x, y) = (10., 10.);

        i = (i + 1) % 45;

        guitar.render(
            &context,
            (x, y),
            Mode(C.natural(), Intervals::CHROMATIC),
            |_| &palette.background,
            None,
            Fill,
        );

        guitar.render(
            &context,
            (x, y),
            Mode(C.natural(), dictionary::pentatonic::minor().intervals()),
            |interval| palette.color(interval),
            Some(Triangle::TopLeft(i as f64 + 10.)),
            Fill,
        );

        guitar.render(
            &context,
            (x, y),
            Mode(C.natural(), dictionary::pentatonic::minor().intervals()),
            |interval| palette.color(interval),
            Some(Triangle::BottomRight(i as f64 + 10.)),
            Fill,
        );

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    document.set_ondrag(Some(
        &Closure::new(Box::new(move || {
            log!("drag");
        }) as Box<dyn FnMut()>)
        .as_ref()
        .unchecked_ref(),
    ));
    document.set_ondrop(Some(
        &Closure::new(Box::new(move || {
            log!("drop");
        }) as Box<dyn FnMut()>)
        .as_ref()
        .unchecked_ref(),
    ));

    // let event_target = EventTarget::new().unwrap();
    let on = Closure::wrap(
        (Box::new(move |event: web_sys::DragEvent| {
            log!("aaaa drop");
        })) as Box<dyn FnMut(_)>,
    );
    document.add_event_listener_with_callback("mousemove", on.as_ref().unchecked_ref());
}

pub type Color = JsValue;

pub use FillOrStroke::*;
#[derive(Copy, Clone, Debug)]
pub enum FillOrStroke {
    Fill,
    Stroke,
}

#[derive(Copy, Clone, Debug)]
pub enum Triangle {
    TopLeft(f64),
    BottomRight(f64),
}

impl Triangle {
    fn path(&self, ctx: &Ctx, b: &Rect) {
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

// #[derive(Copy, Clone, Debug)]
// pub struct Rgb(u8, u8, u8);

// impl Rgb {
//     pub fn js_value(&self) -> JsValue {
//         JsValue::from_str(&format!("rgb({},{},{})", self.0, self.1, self.2))
//     }

//     fn js_value_complicated(&self) -> JsValue {
//         let mut str = [
//             b'r', b'g', b'b', b'(', b'?', b'?', b'?', b',', b'?', b'?', b'?', b',', b'?', b'?',
//             b'?', b')',
//         ];

//         let str = unsafe {
//             debug_assert!(str.len() == 16);
//             Self::write_u8(self.0, str.get_unchecked_mut(4..7));
//             Self::write_u8(self.1, str.get_unchecked_mut(8..1));
//             Self::write_u8(self.2, str.get_unchecked_mut(12..1));
//             debug_assert!(std::str::from_utf8(&str[..]).is_ok());
//             std::str::from_utf8_unchecked(&str[..])
//         };

//         JsValue::from_str(str)
//     }

//     fn write_u8(byte: u8, dest: &mut [u8]) {
//         fn go(byte: u8, dest: &mut u8, it: impl IntoIterator<Item = (u8, u8)>) -> u8 {
//             for (sub, char) in it {
//                 if let Some(byte) = byte.checked_sub(sub) {
//                     *dest = char;
//                     return byte;
//                 }
//             }
//             *dest = b'0';
//             return byte;
//         }

//         let hundreds = [(200, b'2'), (100, b'1')];
//         let tens = [
//             (90, b'9'),
//             (80, b'8'),
//             (70, b'7'),
//             (60, b'6'),
//             (50, b'5'),
//             (40, b'4'),
//             (30, b'3'),
//             (20, b'2'),
//             (10_u8, b'1'),
//         ];
//         let units = [
//             (9, b'9'),
//             (8, b'8'),
//             (7, b'7'),
//             (6, b'6'),
//             (5, b'5'),
//             (4, b'4'),
//             (3, b'3'),
//             (2, b'2'),
//             (1, b'1'),
//         ];

//         unsafe {
//             debug_assert!(dest.len() > 2);
//             go(
//                 go(
//                     go(byte, dest.get_unchecked_mut(0), hundreds),
//                     dest.get_unchecked_mut(1),
//                     tens,
//                 ),
//                 dest.get_unchecked_mut(2),
//                 units,
//             );
//         }
//     }
// }
