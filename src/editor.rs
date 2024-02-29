use gpui::*;

pub struct LayoutState {
    text_size: gpui::Size<Pixels>,
}



struct EditorElement {}


impl EditorElement {
    fn paint_text(
        &mut self,
        text_bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {

    }

    fn compute_layout(&mut self, bounds: Bounds<Pixels>, cx: &mut ElementContext) -> LayoutState {
        let style = self.style.clone();
        let font_id = cx.text_system().resolve_font(&style.text.font());
        let font_size = style.text.font_size.to_pixels(cx.rem_size());
        let line_height = style.text.line_height_in_pixels(cx.rem_size());
        let em_width = cx
            .text_system()
            .typographic_bounds(font_id, font_size, 'm')
            .unwrap()
            .size
            .width;
        let em_advance = cx
            .text_system()
            .advance(font_id, font_size, 'm')
            .unwrap()
            .width;
        let text_width = bounds.size.width;
        let overscroll = size(em_width, px(0.));
        let text_size = size(text_width, bounds.size.height);
        LayoutState {
            text_size,
        }
    }
}