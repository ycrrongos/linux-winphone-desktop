//! End-user documentation.

macro_rules! markdown_page {
    ($name:ident, $path:literal) => {
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/", $path))]
        pub mod $name {}
    };
}

/// Complete user guide.
pub mod guide {
    markdown_page!(en, "guide/en.md");
    markdown_page!(zh, "guide/zh.md");
}

/// Complete changelog.
pub mod changelog {
    markdown_page!(en, "changelog/en.md");
    markdown_page!(zh, "changelog/zh.md");
}

/// Complete translated README.
pub mod readme {
    markdown_page!(zh, "readme/zh.md");
}
