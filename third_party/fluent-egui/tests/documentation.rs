const PUBLIC_API_SOURCES: &[(&str, &str)] = &[
    ("backdrop.rs", include_str!("../src/winui/backdrop.rs")),
    (
        "collections.rs",
        include_str!("../src/winui/collections.rs"),
    ),
    ("commands.rs", include_str!("../src/winui/commands.rs")),
    ("date_time.rs", include_str!("../src/winui/date_time.rs")),
    ("design.rs", include_str!("../src/winui/design.rs")),
    ("easy.rs", include_str!("../src/winui/easy.rs")),
    ("extended.rs", include_str!("../src/winui/extended.rs")),
    (
        "fundamentals.rs",
        include_str!("../src/winui/fundamentals.rs"),
    ),
    ("gallery.rs", include_str!("../src/winui/gallery.rs")),
    (
        "gallery_settings.rs",
        include_str!("../src/winui/gallery_settings.rs"),
    ),
    ("guidance.rs", include_str!("../src/winui/guidance.rs")),
    (
        "icon_catalog.rs",
        include_str!("../src/winui/icon_catalog.rs"),
    ),
    ("icons.rs", include_str!("../src/winui/icons.rs")),
    ("layout.rs", include_str!("../src/winui/layout.rs")),
    ("mod.rs", include_str!("../src/winui/mod.rs")),
    ("motion.rs", include_str!("../src/winui/motion.rs")),
    ("platform.rs", include_str!("../src/winui/platform.rs")),
    ("shell.rs", include_str!("../src/winui/shell.rs")),
    ("text.rs", include_str!("../src/winui/text.rs")),
    ("theme.rs", include_str!("../src/winui/theme.rs")),
    ("web.rs", include_str!("../src/winui/web.rs")),
    ("widgets.rs", include_str!("../src/winui/widgets.rs")),
];

#[test]
fn every_api_doc_block_contains_a_compiling_example_section() {
    for (file, source) in PUBLIC_API_SOURCES {
        let lines = source.lines().collect::<Vec<_>>();
        let mut index = 0;
        while index < lines.len() {
            if !lines[index].trim_start().starts_with("///") {
                index += 1;
                continue;
            }

            let start = index + 1;
            let mut has_examples = false;
            while index < lines.len() && lines[index].trim_start().starts_with("///") {
                has_examples |= lines[index].contains("# Examples");
                index += 1;
            }
            assert!(
                has_examples,
                "{file}:{start} has an API documentation block without # Examples"
            );
        }
    }
}
