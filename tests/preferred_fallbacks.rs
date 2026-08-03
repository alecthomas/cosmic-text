use cosmic_text::{Attrs, Buffer, Fallback, Family, FontSystem, Metrics, Shaping};
use unicode_script::Script;

#[derive(Debug)]
struct TestFallback;

impl Fallback for TestFallback {
    fn common_fallback(&self) -> &[&'static str] {
        &[]
    }

    fn forbidden_fallback(&self) -> &[&'static str] {
        &["Noto Sans Hebrew", "Noto Sans Arabic"]
    }

    fn script_fallback(&self, _script: Script, _locale: &str) -> &[&'static str] {
        &[]
    }
}

#[derive(Clone, Copy)]
enum TextKind {
    Proportional,
    Monospace,
}

#[test]
fn preferred_fallbacks_are_used_before_platform_fallbacks() {
    let cases = [
        (
            TextKind::Proportional,
            Family::SansSerif,
            "Noto Sans Hebrew",
            "א",
        ),
        (
            TextKind::Monospace,
            Family::Monospace,
            "Noto Sans Arabic",
            "ش",
        ),
    ];

    for (kind, primary, expected_family, text) in cases {
        let mut font_system = test_font_system();
        let fallbacks = [
            Family::Name("Unavailable Test Font"),
            Family::Name(expected_family),
        ];

        match kind {
            TextKind::Proportional => font_system.set_proportional_fallbacks(fallbacks),
            TextKind::Monospace => font_system.set_monospace_fallbacks(fallbacks),
        }

        let expected_id = font_system
            .db()
            .query(&cosmic_text::fontdb::Query {
                families: &[Family::Name(expected_family)],
                ..cosmic_text::fontdb::Query::default()
            })
            .expect("fallback font must be loaded");
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(14.0, 20.0));

        let actual_ids = {
            let mut buffer = buffer.borrow_with(&mut font_system);
            buffer.set_size(Some(200.0), Some(50.0));
            buffer.set_text(text, &Attrs::new().family(primary), Shaping::Advanced, None);
            buffer.shape_until_scroll(true);
            buffer
                .layout_runs()
                .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.font_id))
                .collect::<Vec<_>>()
        };

        assert!(!actual_ids.is_empty());
        assert!(actual_ids.iter().all(|id| *id == expected_id));
    }
}

fn test_font_system() -> FontSystem {
    let mut db = cosmic_text::fontdb::Database::new();

    for path in [
        "fonts/Inter-Regular.ttf",
        "fonts/FiraMono-Medium.ttf",
        "fonts/NotoSansHebrew.ttf",
        "fonts/NotoSansArabic.ttf",
    ] {
        db.load_font_data(std::fs::read(path).expect("test font must be readable"));
    }

    db.set_sans_serif_family("Inter");
    db.set_monospace_family("Fira Mono");

    FontSystem::new_with_locale_and_db_and_fallback("en-US".into(), db, TestFallback)
}
