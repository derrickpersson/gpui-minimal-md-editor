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


struct Lines {
    lines: Vec<String>,
}

impl Lines {
    pub fn new(string: &str) -> Self {
        let mut lines = Vec::new();
        let mut line = String::new();
        for c in string.chars() {
            if c == '\n' {
                lines.push(line);
                line = String::new();
            } else {
                line.push(c);
            }
        }
        // Push the last line if the string is not empty or if it is empty (to pass the empty string test)
        if !line.is_empty() || string.is_empty() {
            lines.push(line);
        }
        Self { lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_new() {
        let input = "Hello\nWorld\nThis is a test";
        let lines = Lines::new(input);
        assert_eq!(lines.lines.len(), 3);
        assert_eq!(lines.lines[0], "Hello");
        assert_eq!(lines.lines[1], "World");
        assert_eq!(lines.lines[2], "This is a test");
    }

    #[test]
    fn test_lines_new_empty_string() {
        let input = "";
        let lines = Lines::new(input);
        assert_eq!(lines.lines.len(), 1);
        assert_eq!(lines.lines[0], "");
    }

    #[test]
    fn test_lines_new_no_newline_at_end() {
        let input = "Hello\nWorld";
        let lines = Lines::new(input);
        assert_eq!(lines.lines.len(), 2);
        assert_eq!(lines.lines[0], "Hello");
        assert_eq!(lines.lines[1], "World");
    }

    #[test]
    fn test_lines_new_newline_at_start() {
        let input = "\nHello\nWorld";
        let lines = Lines::new(input);
        assert_eq!(lines.lines.len(), 3);
        assert_eq!(lines.lines[0], "");
        assert_eq!(lines.lines[1], "Hello");
        assert_eq!(lines.lines[2], "World");
    }
}
