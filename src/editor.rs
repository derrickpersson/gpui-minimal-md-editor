use std::sync::Arc;

use gpui::*;
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
}


pub struct EditorElement {
    editor: View<Editor>,
}

impl Editor {
    pub fn new(cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();
        let fh = focus_handle.clone();
        let model = cx.new_model(|_| PieceTable::new("Hello World!"));
        Editor { focus_handle, buffer: model }
    }
}

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering editor!");
        EditorElement::new(cx.view())
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
        std::dbg!("Bounds given to Compute_layout {}", bounds);
        std::dbg!("Isn't bounds supposed to be > 0? Is this the problem?");
        assert!(bounds.size.width > px(0.0) && bounds.size.height > px(0.0));
        self.editor.update(cx,|_editor, cx| {
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
            LayoutState {
                text_size,
                line_height,
                position_map: Arc::new(PositionMap {
                    size: bounds.size,
                    scroll_max: point(0., 0.),
                    scroll_position: point(px(0.), px(0.)),
                    line_layouts: self.layout_lines(text_width, cx),
                    line_height,
                    em_width,
                    em_advance,
                }),
            }
        })
    }

    fn layout_lines(&self, text_width: Pixels, cx: &ViewContext<Editor>) -> Vec<EditorLine> {
        let mut layouts =Vec::new();
        // let content = self.editor.read(cx).buffer.read(cx).content();
        let content = String::from("Hello from layout_lines!\nThis is a test!");
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
        cx.with_content_mask(Some(ContentMask {
            bounds: text_bounds,
        }), |cx| {
            cx.with_z_index(1, |cx| {
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
            });
            
            // cx.with_z_index(1, |cx| {
            //     cursor.paint(content_origin, cx);
            // });
        })

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

    // fn compute_layout(&mut self, bounds: Bounds<Pixels>, cx: &mut ElementContext) -> LayoutState {
    //     let style = self.style.clone();
    //     let font_id = cx.text_system().resolve_font(&style.text.font());
    //     let font_size = style.text.font_size.to_pixels(cx.rem_size());
    //     let line_height = style.text.line_height_in_pixels(cx.rem_size());
    //     let em_width = cx
    //         .text_system()
    //         .typographic_bounds(font_id, font_size, 'm')
    //         .unwrap()
    //         .size
    //         .width;
    //     let em_advance = cx
    //         .text_system()
    //         .advance(font_id, font_size, 'm')
    //         .unwrap()
    //         .width;
    //     let text_width = bounds.size.width;
    //     let overscroll = size(em_width, px(0.));
    //     let text_size = size(text_width, bounds.size.height);
    //     LayoutState {
    //         text_size,
    //     }
    // }