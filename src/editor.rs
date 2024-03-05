use std::ops::Range;

use gpui::*;

use super::piece_table::PieceTable;
use super::editor_element::EditorElement;

#[derive(Clone)]
pub struct Editor {
    focus_handle: FocusHandle,
    pub buffer: Model<PieceTable>,
    pub selection_range: Option<Range<usize>>,
    pub cursor_point: Point<usize>,
}



impl Editor {
    pub fn new(buffer: &Model<PieceTable>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let fh = focus_handle.clone();
        let _ = cx.observe(buffer, Self::on_buffer_changed).detach();
        Editor { 
            focus_handle, 
            buffer: buffer.clone(),
            selection_range: None,
            cursor_point: Point::new(0, 0),
        }
    }

    pub fn key_context(&self, cx: &AppContext) -> KeyContext {
        let mut key_context = KeyContext::default();
        key_context.add("Editor");
        key_context
    }

    fn on_buffer_changed(&mut self, _: Model<PieceTable>, cx: &mut ViewContext<Self>) {
        std::dbg!("Buffer changed!");
        cx.notify();
    }

    pub fn set_selection(&mut self, range: Range<usize>, cx: &mut ViewContext<Self>) {
        self.selection_range = Some(range);
        cx.notify();
    }

    pub fn set_cursor(&mut self, point: Point<usize>, cx: &mut ViewContext<Self>) {
        self.cursor_point = point;
        cx.notify();
    }
}

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering editor!");
        EditorElement::new(cx.view())
    }
}

impl ViewInputHandler for Editor {
    fn text_for_range(&mut self, range: Range<usize>, cx: &mut ViewContext<Editor>) -> Option<String> {
        self.buffer.read(cx).content().get(range.clone()).map(|s| s.to_string())
    }

    fn selected_text_range(&mut self, cx: &mut ViewContext<Editor>) -> Option<Range<usize>> {
        // Assuming `self` has a field `selection_range` representing the selected text
        self.selection_range.clone()
    }

    fn marked_text_range(&self, cx: &mut ViewContext<Editor>) -> Option<Range<usize>> {
        // Assuming `self` has a field `marked_range` representing the marked text
        std::dbg!("marked_text_range", &self.selection_range);
        self.selection_range.clone()
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Editor>) {
        // Assuming `self` has a method to unmark text
        std::dbg!("unmark_text");
    }

    fn replace_text_in_range(&mut self, range: Option<Range<usize>>, text: &str, cx: &mut ViewContext<Editor>) {

        std::dbg!("Trying to update a range of text: {}", &range, &text);
        cx.emit(EditorEvent::InputHandled {
            range: range.unwrap_or(self.selection_range.clone().unwrap_or(0..0)),
            text: text.to_string(),
        });
    }

    fn replace_and_mark_text_in_range(&mut self, range: Option<Range<usize>>, text: &str, mark_range: Option<Range<usize>>, cx: &mut ViewContext<Editor>) {
        // Assuming `self.buffer` has a method to replace and mark text in a given range
        unimplemented!()
    }

    fn bounds_for_range(&mut self, range: Range<usize>, element_bounds: Bounds<Pixels>, cx: &mut ViewContext<Editor>) -> Option<Bounds<Pixels>> {
        std::dbg!("Bounds for range: {:?}", range);
        let style = Style::default();
        let font_id = cx.text_system().resolve_font(&TextStyle::default().font());
        let font_size = style.text.font_size.unwrap_or(TextStyle::default().font_size).to_pixels(cx.rem_size());
        let line_height = phi().to_pixels(rems(1.0).into(), cx.rem_size()).round();
        let em_width = cx
            .text_system()
            .typographic_bounds(font_id, font_size, 'm')
            .unwrap()
            .size
            .width;
        
        Some(Bounds {
            origin: element_bounds.origin, // TODO - adjust based on x, y of point of range.
            size: size(em_width, line_height),
        })
    }
}


#[derive(Clone, Debug)]
pub enum EditorEvent {
    InputHandled {
        range: Range<usize>,
        text: String,
    },
    Movement(CursorSelectionMovement),
}

#[derive(Debug, Clone)]
pub enum CursorSelectionMovement {
    Left,
    Right,
    Up,
    Down,
}

impl EventEmitter<EditorEvent> for Editor {}

impl FocusableView for Editor {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}