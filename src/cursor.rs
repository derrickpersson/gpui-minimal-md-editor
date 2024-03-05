use gpui::*;

pub struct Cursor {
    origin: gpui::Point<Pixels>,
    line_height: Pixels,
    color: Hsla,
}

impl Cursor {
    pub fn new(
        origin: gpui::Point<Pixels>,
        // line_height: Pixels,
        // color: Hsla,
    ) -> Cursor {
        Self {
            origin,
            line_height: Pixels(20.0),
            color: Hsla::blue(),
        }
    }

    pub fn bounding_rect(&self, origin: gpui::Point<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin + origin,
            size: size(px(2.0), self.line_height),
        }
    }

    pub fn paint(&self, origin: gpui::Point<Pixels>, cx: &mut ElementContext) {
        let bounds = Bounds {
            origin: self.origin + origin,
            size: size(px(2.0), self.line_height),
        };
        let cursor = fill(bounds, self.color);
        cx.paint_quad(cursor)
    }
}