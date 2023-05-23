use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub r: f64,
    pub b: f64,
    pub clip: Option<Triangle>,
}

impl Rect {
    pub fn bounding(&self) -> (f64, f64, f64, f64) {
        let (x, y, w, h, r) = (self.x, self.y, self.w, self.h, self.r);

        (x - r / 2., y - r / 2., w + r, h + r)
    }

    pub fn render(
        &self,
        ctx: &Ctx,
        color: &Color,
        clip: Option<Triangle>,
        fill_or_stroke: FillOrStroke,
    ) {
        ctx.save();

        if let Some(clip) = clip {
            ctx.begin_path();
            clip.path(ctx, self);
            ctx.clip();
        }

        ctx.begin_path();

        match fill_or_stroke {
            Fill => {
                ctx.set_fill_style(color);
                self.rect(ctx);
                ctx.fill();
            }
            Stroke => {
                if self.b >= 1. {
                    ctx.set_line_width(self.b);
                    ctx.set_stroke_style(color);
                    self.rect(ctx);
                    ctx.stroke();
                }
            }
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
