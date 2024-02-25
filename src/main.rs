mod piece_table;

use gpui::*;


use crate::piece_table::PieceTable;

pub struct Buffer {
    text: TextBuffer,
}

pub struct TextBuffer {
    piece_table: PieceTable,
}

impl Global for BufferModel {}

#[derive(Clone)]
pub struct BufferModel {
    pub inner: Model<Buffer>,
}

impl BufferModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            inner: cx.new_model(|_| Buffer { 
                text: TextBuffer { 
                    piece_table: PieceTable::new("") 
                }
        }),
        };
        cx.set_global(this.clone());
        this
    }
    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    })
}

pub struct Workspace {
    state: BufferModel,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = BufferModel::init(cx);
            Workspace { state }
        });
        view
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");

        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(
                        vec![
                            div()
                                .bg(rgb(0x4caf50))
                                .text_color(rgb(0xffffff))
                                .child(
                                    format!("Start writing here...{}", "".to_string())
                                ),
                        ]
                    )
            )            
    }
}
