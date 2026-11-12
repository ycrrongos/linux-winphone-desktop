use std::{env, fs, path::PathBuf};

struct Document {
    source: &'static str,
    generated: &'static str,
    replacements: &'static [(&'static str, &'static str)],
}

const DOCUMENTS: &[Document] = &[
    // The original README remains GitHub/crates.io friendly. rustdoc embeds
    // this generated copy from OUT_DIR instead.
    Document {
        source: "README.md",
        generated: "README.md",
        replacements: &[
            ("PROJECT_GUIDE.md", "documentation/guide/en/index.html"),
            ("PROJECT_GUIDE_zh.md", "documentation/guide/zh/index.html"),
            ("README.md", "index.html"),
            ("README_zh.md", "documentation/readme/zh/index.html"),
            ("CHANGELOG.md", "documentation/changelog/en/index.html"),
            ("CHANGELOG_zh.md", "documentation/changelog/zh/index.html"),
        ],
    },
    Document {
        source: "docs/guide/en.md",
        generated: "guide/en.md",
        replacements: &[
            ("PROJECT_GUIDE.md", "index.html"),
            ("PROJECT_GUIDE_zh.md", "../zh/index.html"),
            ("README.md", "../../../index.html"),
            ("README_zh.md", "../../readme/zh/index.html"),
            ("CHANGELOG.md", "../../changelog/en/index.html"),
            ("CHANGELOG_zh.md", "../../changelog/zh/index.html"),
        ],
    },
    Document {
        source: "docs/guide/zh.md",
        generated: "guide/zh.md",
        replacements: &[
            ("PROJECT_GUIDE.md", "../en/index.html"),
            ("PROJECT_GUIDE_zh.md", "index.html"),
            ("README.md", "../../../index.html"),
            ("README_zh.md", "../../readme/zh/index.html"),
            ("CHANGELOG.md", "../../changelog/en/index.html"),
            ("CHANGELOG_zh.md", "../../changelog/zh/index.html"),
        ],
    },
    Document {
        source: "docs/changelog/en.md",
        generated: "changelog/en.md",
        replacements: &[
            ("CHANGELOG.md", "index.html"),
            ("CHANGELOG_zh.md", "../zh/index.html"),
            ("README.md", "../../../index.html"),
            ("README_zh.md", "../../readme/zh/index.html"),
            ("PROJECT_GUIDE.md", "../../guide/en/index.html"),
            ("PROJECT_GUIDE_zh.md", "../../guide/zh/index.html"),
        ],
    },
    Document {
        source: "docs/changelog/zh.md",
        generated: "changelog/zh.md",
        replacements: &[
            ("CHANGELOG.md", "../en/index.html"),
            ("CHANGELOG_zh.md", "index.html"),
            ("README.md", "../../../index.html"),
            ("README_zh.md", "../../readme/zh/index.html"),
            ("PROJECT_GUIDE.md", "../../guide/en/index.html"),
            ("PROJECT_GUIDE_zh.md", "../../guide/zh/index.html"),
        ],
    },
    Document {
        source: "docs/readme/zh.md",
        generated: "readme/zh.md",
        replacements: &[
            ("README.md", "../../../index.html"),
            ("README_zh.md", "index.html"),
            ("PROJECT_GUIDE.md", "../../guide/en/index.html"),
            ("PROJECT_GUIDE_zh.md", "../../guide/zh/index.html"),
            ("CHANGELOG.md", "../../changelog/en/index.html"),
            ("CHANGELOG_zh.md", "../../changelog/zh/index.html"),
        ],
    },
];

fn rewrite_markdown_links(mut markdown: String, replacements: &[(&str, &str)]) -> String {
    for &(source, rustdoc) in replacements {
        // Replace link destinations, but do not replace filenames mentioned in
        // prose or code. The second form preserves an existing section anchor.
        markdown = markdown.replace(&format!("]({source})"), &format!("]({rustdoc})"));
        markdown = markdown.replace(&format!("]({source}#"), &format!("]({rustdoc}#"));
    }
    markdown
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    for document in DOCUMENTS {
        println!("cargo:rerun-if-changed={}", document.source);
        let source = manifest_dir.join(document.source);

        // This permits an incomplete documentation tree during migration. An
        // include_str! referring to a missing generated page still fails with
        // a clear compiler error, so published pages cannot silently vanish.
        if !source.is_file() {
            println!(
                "cargo:warning=optional documentation file is missing: {}",
                source.display()
            );
            continue;
        }

        let markdown = fs::read_to_string(&source)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", source.display()));
        let markdown = rewrite_markdown_links(markdown, document.replacements);
        let destination = out_dir.join(document.generated);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("cannot create {}: {error}", parent.display()));
        }
        fs::write(&destination, markdown)
            .unwrap_or_else(|error| panic!("cannot write {}: {error}", destination.display()));
    }
}
