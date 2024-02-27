use gpui::*;

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
    // focus_handle: FocusHandle,
}

impl RawText {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(| _cx | Self {})
    }
}

impl Render for RawText {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2a2a2a))
            .text_color(rgb(0xffffff))
            .py_2()
            .px_4()
            .child("Displaying RAW TEXT!")
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