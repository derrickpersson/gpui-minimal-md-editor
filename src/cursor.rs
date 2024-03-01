use gpui::*;

pub struct Cursor {
    origin: gpui::Point<Pixels>,
    line_height: Pixels,
    color: Hsla,
}


// impl TrackingCursor {
//     pub fn new() -> Self {
//         Self {
//             selection: 0..0,
//         }
//     }

//     pub fn reset(&mut self) {
//         self.selection = 0..0;
//     }

//     pub fn select_all(&mut self, text: &str) {
//         self.selection = 0..text.len();
//     }

//     pub fn move_left(&mut self) {
//         if self.selection.start > 0 {
//             let i = if self.selection.start == self.selection.end {
//                 self.selection.start - 1
//             } else {
//                 self.selection.start
//             };
//             self.selection = i..i;
//         }
//     }

//     pub fn move_right(&mut self) {
//         if self.selection.end < self.selection.end {
//             self.selection = self.selection.start..self.selection.end + 1;
//         }
//     }
// }

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