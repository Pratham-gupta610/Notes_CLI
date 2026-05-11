// export/mod.rs

use std::fs;
use std::io;
use crate::note::{Note, format_timestamp};

pub fn export_markdown(note: &Note, path: &str) -> io::Result<()> {
    let mut content = format!("# {}\n\n", note.title);
    content.push_str(&format!("**ID:** {}  \n", note.id));
    content.push_str(&format!("**Tags:** {}  \n", note.tags.join(", ")));
    content.push_str(&format!("**Color:** {}  \n", note.color.as_str()));
    content.push_str(&format!("**Created:** {}  \n", format_timestamp(note.created_at)));
    content.push_str(&format!("**Updated:** {}  \n", format_timestamp(note.updated_at)));
    content.push_str(&format!("**Versions:** {}  \n\n", note.versions.len()));
    if !note.links.is_empty() {
        let linked: Vec<String> = note.links.iter().map(|id| format!("[[{}]]", id)).collect();
        content.push_str(&format!("**Linked Notes:** {}  \n\n", linked.join(", ")));
    }
    content.push_str("---\n\n");

    for v in &note.versions {
        content.push_str(&format!("## Version {} _({}words, {}chars, {})_\n\n",
            v.version_number, v.word_count(), v.char_count(), format_timestamp(v.created_at)));
        content.push_str(&v.content);
        content.push_str("\n\n");
    }

    fs::write(path, content)
}

pub fn export_plain_text(note: &Note, path: &str) -> io::Result<()> {
    let mut content = format!("NOTE: {}\n", note.title);
    content.push_str(&format!(
        "ID: {} | Tags: {} | Color: {} | Versions: {} | Created: {} | Updated: {}\n",
        note.id, note.tags.join(", "), note.color.as_str(), note.versions.len(),
        format_timestamp(note.created_at), format_timestamp(note.updated_at)
    ));
    content.push_str(&"=".repeat(50));
    content.push('\n');

    for v in &note.versions {
        content.push_str(&format!(
            "\nVersion {} ({} words, {} chars, {}):\n{}\n",
            v.version_number, v.word_count(), v.char_count(),
            format_timestamp(v.created_at), v.content
        ));
    }

    fs::write(path, content)
}

/// Import a plain .md or .txt file as a new note version.
/// Returns the content string so the caller can add_version() it.
pub fn import_markdown(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}