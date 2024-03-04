use gpui::*;

use super::piece_table::PieceTable;
use super::editor::*;

pub struct Workspace {
    editor: Editor,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view: View<Workspace> = cx.new_view(|cx| {
            let model = cx.new_model(|_| PieceTable::new(""));
            let editor = Editor::new(model, cx);
            // let _ = cx.observe(&model, |workspace, model, cx| {
            //     std::dbg!("Model updated! We should do something... {}", model.read(cx).content());
            //     // TODO - Recreate the editor model / view here.
            //     // cx.update_view(&editor, cx);// -> Works, but not the right way I'm pretty sure.
            // }).detach();
            Workspace {
                editor,
            }
        });
        view
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering Workspace view");
        div()
            .size_full()
            .py_2()
            .px_4()
            .bg(rgb(0x333333))
            .child(cx.new_view(| cx| {
                let editor = self.editor.clone();
                // TODO - there's a better way to do this:
                // Automatically focus the editor. 
                editor.focus_handle(cx).focus(cx);
                editor
            }))
    }
}


pub fn run_app(app: App) {
    app.run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    })
}