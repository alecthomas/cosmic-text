use cosmic_text::{fontdb, Attrs, Buffer, FontSystem, Metrics, Shaping, Wrap};

#[test]
fn monospace_width_snaps_proportional_glyphs_to_whole_cells() {
    let mut font_system =
        FontSystem::new_with_locale_and_db("en-US".into(), fontdb::Database::new());
    font_system
        .db_mut()
        .load_font_data(std::fs::read("fonts/Inter-Regular.ttf").unwrap());
    let cell = 8.0;
    let mut buffer = Buffer::new(&mut font_system, Metrics::new(14.0, 20.0));
    buffer.set_wrap(Wrap::None);
    buffer.set_monospace_width(Some(cell));
    buffer.set_text("iWi.m", &Attrs::new(), Shaping::Advanced, None);
    buffer.shape_until_scroll(&mut font_system, false);

    let run = buffer.layout_runs().next().unwrap();
    let cells: Vec<(f32, f32)> = run
        .glyphs
        .iter()
        .map(|glyph| (glyph.x / cell, glyph.w / cell))
        .collect();
    // Narrow glyphs keep one cell instead of rounding to none; W spans two.
    assert_eq!(
        vec![(0.0, 1.0), (1.0, 2.0), (3.0, 1.0), (4.0, 1.0), (5.0, 2.0)],
        cells
    );
    assert_eq!(7.0 * cell, run.line_w);
}
