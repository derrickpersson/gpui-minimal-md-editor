use gpui::*;

use super::piece_table::PieceTable;
use super::editor::*;

pub struct Workspace {
    editor: View<Editor>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view: View<Workspace> = cx.new_view(|cx | {
            let model = cx.new_model(|_| PieceTable::new(""));
            let editor = cx.new_view(|cx| {
                let editor = Editor::new(&model, cx);
                editor
            });

            cx.subscribe(&editor, move |workspace, view, evt,  cx| {
                match evt {
                    EditorEvent::InputHandled { range, text } => {
                        std::dbg!("Content changed", range, text);
                        cx.update_model( &model, | model: &mut PieceTable, cx | {
                            model.replace(range.start, range.end, text);
                            cx.notify();
                        });
                    },
                    _ => {}
                }
            }).detach();

            Workspace {
                editor,
            }
        });
        view
    }

    // pub fn update_buffer(&mut self, buffer: Model<PieceTable>, cx: &mut ViewContext<Self>) {
    //     cx.update_model(buffer, |model, cx| {
    //         model.replace(0, model.content().len(), buffer);
    //     });
    // }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering Workspace view");
        let editor = self.editor.clone();
                // TODO - there's a better way to do this:
                // Automatically focus the editor. 
                editor.focus_handle(cx).focus(cx);
                
        div()
            .size_full()
            .py_2()
            .px_4()
            .bg(rgb(0x333333))
            .child(editor)
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