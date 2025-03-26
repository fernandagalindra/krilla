use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::MetadataProvider;

use crate::geom::Path;
use crate::geom::Transform;
use crate::surface::Surface;
use crate::text::GlyphId;
use crate::text::{Font, PaintMode};

pub(crate) fn glyph_path(font: Font, glyph: GlyphId) -> Option<tiny_skia_path::Path> {
    let outline_glyphs = font.font_ref().outline_glyphs();
    let mut outline_builder = OutlineBuilder::new();

    if let Some(outline_glyph) = outline_glyphs.get(glyph.to_skrifa()) {
        outline_glyph
            .draw(
                DrawSettings::unhinted(skrifa::instance::Size::unscaled(), font.location_ref()),
                &mut outline_builder,
            )
            .ok()?;
    }

    outline_builder.finish()
}

/// Draw an outline-based glyph on a surface.
pub(crate) fn draw_glyph(
    font: Font,
    glyph: GlyphId,
    paint_mode: PaintMode,
    base_transform: Transform,
    surface: &mut Surface,
) -> Option<()> {
    let path = Path(glyph_path(font, glyph).and_then(|p| p.transform(base_transform.to_tsp()))?);

    let old_fill = surface.get_fill().cloned();
    let old_stroke = surface.get_stroke().cloned();

    match paint_mode {
        PaintMode::Fill(f) => {
            surface.set_fill(Some(f.clone()));
            surface.set_stroke(None);
        }
        PaintMode::Stroke(s) => {
            surface.set_fill(None);
            surface.set_stroke(Some(s.clone()));
        }
        PaintMode::FillStroke(f, s) => {
            surface.set_fill(Some(f.clone()));
            surface.set_stroke(Some(s.clone()));
        }
    }

    surface.draw_path(&path);

    surface.set_fill(old_fill);
    surface.set_stroke(old_stroke);

    Some(())
}

/// A wrapper struct for implementing the `OutlinePen` trait.
pub(crate) struct OutlineBuilder(tiny_skia_path::PathBuilder);

impl OutlineBuilder {
    pub(crate) fn new() -> Self {
        Self(tiny_skia_path::PathBuilder::new())
    }

    pub(crate) fn finish(self) -> Option<tiny_skia_path::Path> {
        self.0.finish()
    }
}

impl OutlinePen for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0.quad_to(cx0, cy0, x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0.cubic_to(cx0, cy0, cx1, cy1, x, y);
    }

    fn close(&mut self) {
        self.0.close()
    }
}
