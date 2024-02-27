use gpui::*;
use super::state::*;

pub struct Workspace {
    text_view: View<RawText>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let text_view = RawText::build(cx);
            Workspace { text_view }
        });
        view
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering Workspace view");
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x333333))
            .justify_center()
            .items_center()
            .child(self.text_view.clone())
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

impl RawText {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let focus_handle = cx.focus_handle();
        let fh = focus_handle.clone();
        let view = cx.new_view(move |cx| {
            let model = cx.global::<StateModel>().clone();
            let content = model.inner.read(cx).text.content();
            cx.observe(&model.inner, |this: &mut RawText, model, cx| {
                this.content = model.read(cx).text.content();
                cx.notify();
            })
            .detach();
            cx.on_focus(&fh, |_, _cx| {
                std::dbg!("Focused!");
            })
            .detach();
            cx.on_blur(&fh, |_, cx| {
                std::dbg!("Blurred!!");
                cx.hide();
            })
            .detach();
            Self {
                focus_handle,
                content,
                model,
            }
        });
        view
    }
}

impl EventEmitter<TextEvent> for RawText {}

impl Render for RawText {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);
        let model = self.model.clone();
        // TODO: Implement cursor model, for now, always set cursor to the end of the text
        let cursor_position = model.inner.read(cx).text.content().len(); 
        div()
            .track_focus(&self.focus_handle)
            .flex()
            .bg(rgb(0x2a2a2a))
            .text_color(rgb(0xffffff))
            .py_2()
            .px_4()
            .on_key_down( move |event, window_context| {
                model.inner.update(window_context, |state, model_context| {

                    model_context.emit(TextEvent::Input { 
                        keystroke: event.keystroke.clone(),
                        position: cursor_position.clone(),
                    });
                    // let keystroke = &event.keystroke.key;
                    // // TODO: Handle special key strokes (i.e. space, backspace, etc.)
                    // // TODO: Implement CURSOR model, handle cursor movement
                    // model.text.insert(0, keystroke);
                    // std::dbg!(&keystroke);
                    // std::dbg!(&model.text.content());
                    // model_context.notify();
                });
            })
            .child(format!("{}", self.content))
            // .cursor(CursorStyle::PointingHand)
            // .on_mouse_down(MouseButton::Left, |_mde, cx| {
            //     StateModel::update(
            //         |model, cx| {
            //             cx.update_model(&model.inner, |_model, cx| {
            //                 let message = OutgoingMessage {
            //                     message: "Hello from the other side".into(),
            //                 };
            //                 cx.emit(message);
            //             })
            //         },
            //         cx,
            //     );
            // })
    }
}