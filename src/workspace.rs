use gpui::*;
use super::state::*;
use super::editor::*;

pub struct Workspace {
    editor: Editor,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let editor = Editor::new(cx);
            Workspace { editor }
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
                Editor::new(cx)
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

struct RawText {
    focus_handle: FocusHandle,
    pub content: String,
    pub model: StateModel,
}

// impl RawText {
//     pub fn build(cx: &mut WindowContext) -> View<Self> {
//         let focus_handle = cx.focus_handle();
//         let fh = focus_handle.clone();
//         let view = cx.new_view(move |cx| {
//             let model = cx.global::<StateModel>().clone();
//             let content = model.inner.read(cx).text.content();
//             cx.observe(&model.inner, |this: &mut RawText, model, cx| {
//                 this.content = model.read(cx).text.content();
//                 cx.notify();
//             })
//             .detach();
//             cx.on_focus(&fh, |_, _cx| {
//                 std::dbg!("Focused!");
//             })
//             .detach();
//             cx.on_blur(&fh, |_, cx| {
//                 std::dbg!("Blurred!!");
//                 cx.hide();
//             })
//             .detach();
//             Self {
//                 focus_handle,
//                 content,
//                 model,
//             }
//         });
//         view
//     }
// }

impl EventEmitter<TextEvent> for RawText {}

// impl Render for RawText {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        
        // TODO: Implement cursor model, for now, always set cursor to the end of the text
        // div()
        //     .py_4()
        //     .px_16()
        //     .w_full()
        //     .h_full()
        //     .bg(rgb(0x333333))
        //     .text_color(rgb(0xffffff))
        //     .track_focus(&self.focus_handle)
        //     .cursor_text()
        //     .on_key_down( move |event, window_context| {
        //         model.inner.update(window_context, |state, model_context| {

        //             // TODO: Handle special key strokes (i.e. space, backspace, etc.)
        //             // TODO: Implement CURSOR model, handle cursor movement

        //             model_context.emit(TextEvent::Input { 
        //                 keystroke: event.keystroke.clone(),
        //                 position: cursor_position.clone(),
        //             });
        //         });
        //     })
        //     .child(format!("{}", self.content))
            
//     }
// }