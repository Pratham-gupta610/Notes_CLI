// search/mod.rs

use crate::note::{Note, Version};

/// Search versions by content keyword
pub fn search_by_content<'a>(notes: &'a [Note], query: &str) -> Vec<(usize, &'a Version)> {
    let q = query.to_lowercase();
    let mut results = Vec::new();
    for (i, note) in notes.iter().enumerate() {
        if let Some(v) = note.versions.iter().rev().find(|v| v.content.to_lowercase().contains(&q)) {
            results.push((i, v));
        }
    }
    results
}

/// Search notes by tag
pub fn search_by_tag(notes: &[Note], query: &str) -> Vec<usize> {
    let q = query.to_lowercase();
    notes.iter().enumerate()
        .filter(|(_, n)| n.tags.iter().any(|t| t.to_lowercase().contains(&q)))
        .map(|(i, _)| i)
        .collect()
}

/// Search notes by title
pub fn search_by_title(notes: &[Note], query: &str) -> Vec<usize> {
    let q = query.to_lowercase();
    notes.iter().enumerate()
        .filter(|(_, n)| n.title.to_lowercase().contains(&q))
        .map(|(i, _)| i)
        .collect()
}

/// Full-text search across title, tags, and all version content
pub fn search_full_text(notes: &[Note], query: &str) -> Vec<usize> {
    let q = query.to_lowercase();
    notes.iter().enumerate()
        .filter(|(_, n)| {
            n.title.to_lowercase().contains(&q)
            || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
            || n.versions.iter().any(|v| v.content.to_lowercase().contains(&q))
        })
        .map(|(i, _)| i)
        .collect()
}