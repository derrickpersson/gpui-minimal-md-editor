use gpui::*;

use crate::actions::MoveLeft;

use super::piece_table::PieceTable;
use super::editor::*;
// use super::keymap::default_keymap;

pub struct Workspace {
    editor: View<Editor>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        // let key_bindings = default_keymap();
        let key_bindings = vec![KeyBinding::new(
            "left",
            MoveLeft,
            Some("editor")
        )];

        // Example keymap:
        // let keymap = Keymap::new(vec![KeyBinding::new(
        //     "cmd-n",
        //     TestAction,
        //     Some("ProjectPanel"),
        // )]);

        std::dbg!("Key bindings: {:?}", &key_bindings);

        cx.bind_keys(key_bindings);

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
                        cx.update_view(&view, |view, cx| {
                            view.set_selection(range.start + text.len()..range.start + text.len(), cx);
                            view.set_cursor(Point {
                                x: view.selection.head.x + text.len(),
                                y: 0,
                            }, cx);
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
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering Workspace view");
        let editor = self.editor.clone();
                // TODO - there's probably a better way to do this:
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