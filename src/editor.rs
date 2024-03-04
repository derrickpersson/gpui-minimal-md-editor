use std::{ops::Range, sync::Arc};

use gpui::*;
use crate::cursor::Cursor;

use super::piece_table::PieceTable;

pub struct PositionMap {
    size: Size<Pixels>,
    line_height: Pixels,
    scroll_position: gpui::Point<Pixels>,
    scroll_max: gpui::Point<f32>,
    em_width: Pixels,
    em_advance: Pixels,
    line_layouts: Vec<EditorLine>,
}

pub struct EditorLine {
    pub line: ShapedLine,
}

impl EditorLine {
    fn draw(&self, layout: &LayoutState, row: u32, content_origin: gpui::Point<Pixels>, cx: &mut ElementContext) {
        let line_origin = content_origin + gpui::Point::new(px(0.), layout.line_height * row as f32);
        std::dbg!("Drawing line at: {:?}", line_origin);
        self.line.paint(line_origin, layout.line_height, cx).unwrap()
    }
}


pub struct LayoutState {
    /// Super important!
    position_map: Arc<PositionMap>,
    /// Size of the text area
    text_size: gpui::Size<Pixels>,
    line_height: Pixels,
}


#[derive(Clone)]
pub struct Editor {
    focus_handle: FocusHandle,
    buffer: Model<PieceTable>,
    selection_range: Option<Range<usize>>,
}


pub struct EditorElement {
    editor: View<Editor>,
}

impl Editor {
    pub fn new(cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();
        let fh = focus_handle.clone();
        let model = cx.new_model(|_| PieceTable::new(""));
        let _ = cx.observe(&model, |model, cx| {
            std::dbg!("Model updated! We should do something... {}", model.read(cx).content());
            // TODO - Recreate the editor model / view here.
            cx.refresh(); // -> Works, but not the right way I'm pretty sure.
        }).detach();
        Editor { 
            focus_handle, 
            buffer: model,
            selection_range: Some(0..0),
        }
    }

    fn key_context(&self, cx: &AppContext) -> KeyContext {
        let mut key_context = KeyContext::default();
        key_context.add("Editor");
        key_context
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
        
        self.buffer.update(cx, |buffer, mod_cx| {
            std::dbg!("Updating text in range!");
            let range = range.unwrap_or(0..0);
            buffer.replace(range.start, range.end, text);
            mod_cx.notify();
            // mod_cx.emit(EditorEvent::InputHandled {
            //     range,
            //     text: text.to_string(),
            // });
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

impl EditorElement {
    pub fn new(editor: &View<Editor>) -> Self {
        Self { editor: editor.clone() }
    }

    fn register_actions(&self, cx: &mut WindowContext) {

        // TODO: Register different actions that can be taken, i.e. undo, redo, etc.
    }


    fn compute_layout(&mut self, bounds: Bounds<Pixels>, cx: &mut ElementContext) -> LayoutState {
        self.editor.update(cx,|editor, cx| {
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
            let em_advance = cx
                .text_system()
                .advance(font_id, font_size, 'm')
                .unwrap()
                .width;
            let text_width = px(600.0);
            let overscroll = size(em_width, px(0.));
            let text_size = size(text_width, px(600.0));
            let content = editor.buffer.read(cx).content().clone();
            std::dbg!("Content: {}", &content);
            LayoutState {
                text_size,
                line_height,
                position_map: Arc::new(PositionMap {
                    size: bounds.size,
                    scroll_max: point(0., 0.),
                    scroll_position: point(px(0.), px(0.)),
                    line_layouts: self.layout_lines(&content, text_width, cx),
                    line_height,
                    em_width,
                    em_advance,
                }),
            }
        })
    }

    fn layout_lines(&self, content: &str, text_width: Pixels, cx: &ViewContext<Editor>) -> Vec<EditorLine> {
        let mut layouts =Vec::new();
        let lines: Vec<String> = content.split('\n').map(|line| line.to_string()).collect();
        for line in lines {
            let font_id = cx.text_system().resolve_font(&TextStyle::default().font());
            let font_size = TextStyle::default().font_size.to_pixels(cx.rem_size());
            let wrap_width = text_width; // or another appropriate width value
            let text_runs: Vec<TextRun> = vec![TextRun {
                len: line.len().saturating_sub(1),
                font: TextStyle::font(&TextStyle::default()),
                color: Hsla::white(),
                background_color: TextStyle::default().background_color,
                underline: TextStyle::default().underline,
                strikethrough: TextStyle::default().strikethrough,
            }];
            std::dbg!("Shaping line: {}", &line, wrap_width, font_size, &text_runs);
            let shaped_line = cx.text_system().shape_line(
                line.into(),
                font_size,
                &text_runs,
            ).unwrap();
            layouts.push(EditorLine {
                line: shaped_line,
            });
        }
        layouts
    }

    fn paint_text(
        &mut self,
        text_bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let start_row = 0;
        let content_origin = text_bounds.origin;
        for (ix, editor_line) in
            layout.position_map.line_layouts.iter().enumerate()
        {
            let row = start_row + ix as u32;
            std::dbg!("Drawing line: {}", row);
            editor_line.draw(
                layout,
                row,
                content_origin,
                cx,
            )
        }
        let cursor = Cursor::new(
            Point::new(px(0.), px(0.)),
        );
        
        cx.with_z_index(1, |cx| {
            std::dbg!("Painting cursor! {}", content_origin);
            cursor.paint(content_origin, cx)
        });
    }
}


impl Element for EditorElement {
    type State = ();

    fn request_layout(
            &mut self,
            state: Option<Self::State>,
            cx: &mut ElementContext,
        ) -> (LayoutId, Self::State) {
            // TODO: Something more here?
            (cx.with_element_context(|cx| cx.request_layout(&Style::default(), None)), ())
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, _element_state: &mut Self::State, cx: &mut ElementContext) {
        let editor = self.editor.clone();
    
        cx.paint_view(self.editor.entity_id(), |cx| {
            cx.with_text_style(Some(TextStyleRefinement {
                ..Default::default()
            }), |cx| {
                let mut layout = self.compute_layout(bounds, cx);
                let text_bounds = Bounds {
                    origin: bounds.origin,
                    size: layout.text_size,
                };

                let focus_handle = editor.focus_handle(cx);
                let key_context = editor.read(cx).key_context(cx);

                cx.with_key_dispatch(Some(key_context), Some(focus_handle.clone()), |fh, cx| {
                    cx.handle_input(
                        &focus_handle,
                        ElementInputHandler::new(text_bounds, self.editor.clone()),
                    );
                });
                std::dbg!("Painting editor Element Text", text_bounds);
                self.paint_text(bounds, &mut layout, cx)
            })
        })
    }
}

impl IntoElement for EditorElement {
    type Element = Self;
    fn element_id(&self) -> Option<ElementId> {
        self.editor.element_id()
    }

    fn into_element(self) -> Self::Element {
        self
    }
}