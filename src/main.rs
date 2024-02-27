use gpui::*;

mod state;
mod workspace;
mod piece_table;

use state::StateModel;
use workspace::Workspace;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            StateModel::init(cx);
            let view = Workspace::build(cx);
            view
        });
    })
}