use gpui::*;

mod state;
mod workspace;
mod piece_table;

use state::StateModel;
use workspace::Workspace;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(500.)).into(),
                }),
                ..Default::default()
            }, |cx| {
            StateModel::init(cx);
            let view = Workspace::build(cx);
            view
        });
    })
}