use gpui::*;

struct SimpleGlyph {
    id: GlyphId,
    position: PointK<Pixels>,
    index: usize,
}

struct LineLayout {
    glyphs: Vec<SimpleGlyph>,
    width: Pixels,
    platform_text_system: TextSystem,
}

impl LineLayout {
    /// Return the x position for the glyph at the given index
    pub fn x_for_index(index: usize) -> Pixels {
        return index * fixed_glyph_width
    }

    /// Return the index for the glyph at or before the given x position
    pub fn index_for_x(x: Float) -> usize {
        return floor(x / fixed_glyph_width)
    }

    pub fn layout_line(&self, text: &str, font_size: Pixels, runs: &[TextRun]) {
        self.platform_text_system.layout_line(text, font_size, runs)
    }
}