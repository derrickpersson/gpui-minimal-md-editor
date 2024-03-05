use crate::{actions::MoveLeft, cursor::Cursor};
use gpui::*;
use super::editor::Editor;
use std::{any::TypeId, ops::Range, sync::Arc};


pub struct EditorElement {
    editor: View<Editor>,
}

impl EditorElement {
    pub fn new(editor: &View<Editor>) -> Self {
        Self { editor: editor.clone() }
    }

    pub fn register_actions(&self, cx: &mut WindowContext) {
        std::dbg!("Registering all the actions!");
        let view = &self.editor;
        register_action(view, cx, Editor::selection_move_left);
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
                selection: editor.selection.clone()
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
            std::dbg!("Painting cursor! {}", &layout.selection.head);
            let x = layout.position_map.line_layouts[0].line.x_for_index(layout.selection.head.x);

            cursor.paint(Point {
                x: x + content_origin.x,
                y: content_origin.y,
            }, cx)
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
                std::dbg!("Key context: {:?}", &key_context);

                cx.with_key_dispatch(Some(key_context), Some(focus_handle.clone()), |fh, cx| {
                    self.register_actions(cx);

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
    selection: Selection,
}

#[derive(Clone)]
pub struct Selection {
    pub head: Point<usize>,
    pub range: Option<Range<usize>>,
}


pub fn register_action<T: Action>(
    view: &View<Editor>,
    cx: &mut WindowContext,
    listener: impl Fn(&mut Editor, &T, &mut ViewContext<Editor>) + 'static,
) {
    std::dbg!("Registering action!");
    let view = view.clone();
    cx.on_action(TypeId::of::<T>(), move |action, phase, cx| {
        std::dbg!("Action happened here!", &action);
        let action = action.downcast_ref().unwrap();
        std::dbg!("Action phase!", &phase);
        if phase == DispatchPhase::Bubble {
            view.update(cx, |editor, cx| {
                std::dbg!("Action happened here, going to call the listener with it!");
                listener(editor, action, cx);
            })
        }
    })
}