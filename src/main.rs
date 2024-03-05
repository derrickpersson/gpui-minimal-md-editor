use gpui::*;

mod workspace;
mod piece_table;
mod cursor;
mod editor;
mod editor_element;
mod actions;
mod keymap;

use workspace::Workspace;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(format!("GPUI Markdown Editor {}", VERSION).into()),
                    ..TitlebarOptions::default()
                }),
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(500.)).into(),
                }),
                ..Default::default()
            }, |cx| {
            let view = Workspace::build(cx);
            view
        });
    })
}