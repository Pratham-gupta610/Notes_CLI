// storage/storage.rs

use std::io::Write;
use crate::manager::NoteManager;
use crate::note::{NoteColor, Version};

pub fn save_notes_to_file(nm: &NoteManager, filename: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(filename)?;
    for note in &nm.notes {
        writeln!(file,
            "Note ID: {}, Title: {}, Pinned: {}, Tags: {}, Color: {}, CreatedAt: {}, UpdatedAt: {}, Password: {}, Reminder: {}, Links: {}",
            note.id,
            note.title,
            note.pinned,
            note.tags.join("|"),
            note.color.as_str(),
            note.created_at,
            note.updated_at,
            note.password.as_deref().unwrap_or(""),
            note.reminder.map(|r| r.to_string()).unwrap_or_default(),
            note.links.iter().map(|l| l.to_string()).collect::<Vec<_>>().join("|"),
        )?;
        for version in &note.versions {
            writeln!(file, "Version {}, CreatedAt: {}: {}",
                version.version_number, version.created_at, version.content)?;
        }
    }
    Ok(())
}

pub fn load_notes_from_file(filename: &str) -> std::io::Result<NoteManager> {
    let content = std::fs::read_to_string(filename)?;
    let mut nm = NoteManager::new();

    for line in content.lines() {
        if line.starts_with("Note ID:") {
            // Format: "Note ID: 1, Title: Foo, Pinned: false, Tags: general, Color: blue, CreatedAt: 0, UpdatedAt: 0, Password: , Reminder: , Links: "
            // Also supports legacy: "Note ID: 1, Title: Foo"
            let parts: Vec<&str> = line.splitn(11, ',').collect();

            let id: u32 = parts.get(0)
                .and_then(|s| s.strip_prefix("Note ID:"))
                .map(|s| s.trim())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let title = parts.get(1)
                .and_then(|s| s.trim().strip_prefix("Title:"))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();

            let pinned = parts.get(2)
                .and_then(|s| s.trim().strip_prefix("Pinned:"))
                .map(|s| s.trim() == "true")
                .unwrap_or(false);

            let tags: Vec<String> = parts.get(3)
                .and_then(|s| s.trim().strip_prefix("Tags:"))
                .map(|s| s.trim().split('|').map(|t| t.to_string()).collect())
                .unwrap_or_else(|| vec!["general".to_string()]);

            let color = parts.get(4)
                .and_then(|s| s.trim().strip_prefix("Color:"))
                .map(|s| NoteColor::from_str(s.trim()))
                .unwrap_or(NoteColor::Default);

            let created_at: u64 = parts.get(5)
                .and_then(|s| s.trim().strip_prefix("CreatedAt:"))
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);

            let updated_at: u64 = parts.get(6)
                .and_then(|s| s.trim().strip_prefix("UpdatedAt:"))
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);

            let password: Option<String> = parts.get(7)
                .and_then(|s| s.trim().strip_prefix("Password:"))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            let reminder: Option<u64> = parts.get(8)
                .and_then(|s| s.trim().strip_prefix("Reminder:"))
                .and_then(|s| s.trim().parse().ok());

            let links: Vec<u32> = parts.get(9)
                .and_then(|s| s.trim().strip_prefix("Links:"))
                .map(|s| s.trim().split('|').filter_map(|x| x.parse().ok()).collect())
                .unwrap_or_default();

            nm.create_note(title);
            if let Some(note) = nm.notes.last_mut() {
                note.id         = id;
                note.pinned     = pinned;
                note.tags       = tags;
                note.color      = color;
                note.created_at = created_at;
                note.updated_at = updated_at;
                note.password   = password;
                note.reminder   = reminder;
                note.links      = links;
            }
            if id >= nm.next_id { nm.next_id = id + 1; }

        } else if line.starts_with("Version ") {
            // New format: "Version 1, CreatedAt: 1234567890: content here"
            // Legacy:     "Version 1: content here"
            let (ts, content_str) = if line.contains(", CreatedAt:") {
                // split off "Version N, CreatedAt: TS: content"
                let after_ver = line.trim_start_matches("Version ");
                // after_ver = "1, CreatedAt: 1234567890: content"
                let comma_pos = after_ver.find(',').unwrap_or(after_ver.len());
                let rest = &after_ver[comma_pos..]; // ", CreatedAt: 1234567890: content"
                if let Some(stripped) = rest.strip_prefix(", CreatedAt: ") {
                    // "1234567890: content"
                    let colon_pos = stripped.find(':').unwrap_or(stripped.len());
                    let ts_str = &stripped[..colon_pos];
                    let content = stripped[colon_pos..].trim_start_matches(':').trim();
                    (ts_str.trim().parse::<u64>().unwrap_or(0), content.to_string())
                } else {
                    (0, rest.trim_start_matches(':').trim().to_string())
                }
            } else {
                // Legacy: "Version N: content"
                let content_part = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
                (0, content_part)
            };

            if let Some(note) = nm.notes.last_mut() {
                let vn = note.versions.len() as u32 + 1;
                note.versions.push(Version::new_with_ts(vn, content_str, ts));
                note.updated_at = note.updated_at.max(ts);
            }
        }
    }

    Ok(nm)
}