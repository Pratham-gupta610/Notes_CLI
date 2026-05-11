// manager/note_manager.rs

use crate::note::{Note, NoteColor};
use colored::*;
pub struct NoteManager {
    pub notes: Vec<Note>,
    pub(crate) next_id: u32,
    pub autosave: bool,
    pub trash: Vec<Note>,   // ← undo-delete buffer
}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager {
            notes: Vec::new(),
            next_id: 1,
            autosave: true,
            trash: Vec::new(),
        }
    }

    pub fn create_note(&mut self, title: String) {
        let id = self.next_id;
        self.next_id += 1;
        let note = Note::new(id, title);
        self.notes.push(note);
    }

    pub fn create_note_from_template(&mut self, title: String, template: &str) {
        self.create_note(title);
        if let Some(note) = self.notes.last_mut() {
            note.add_version(template.to_string());
        }
    }

    pub fn get_note_by_id(&mut self, id: u32) -> Option<&mut Note> {
        self.notes.iter_mut().find(|note| note.id == id)
    }

    pub fn get_note_by_id_ref(&self, id: u32) -> Option<&Note> {
        self.notes.iter().find(|note| note.id == id)
    }
pub fn show_all_notes(&self) {
    println!("\n\x1b[1;34m╔══════════════════════════════╗");
    println!("║        📒 MY NOTES           ║");
    println!("╚══════════════════════════════╝\x1b[0m\n");

    if self.notes.is_empty() {
        println!("\x1b[33m  No notes found.\x1b[0m\n");
        return;
    }

    // Pinned first
    let mut sorted: Vec<&Note> = self.notes.iter().collect();
    sorted.sort_by(|a, b| b.pinned.cmp(&a.pinned));

    for note in sorted {
        let color = note.color.to_ansi();
        let reset = "\x1b[0m";
        let pin   = if note.pinned { "\x1b[38;5;214m(Imp)\x1b[0m " } else { "  " };
        let due   = if note.is_reminder_due() { " 🔔" } else { "" };

        println!("{color}  ┌─ {pin}Note #{} ─ {}{due}", note.id, note.title);

        if note.versions.is_empty() {
            println!("{color}  │   (no versions yet){reset}");
        } else {
            let mut first = true;
            for version in note.versions.iter().rev() {
                let wc = version.word_count();
                let tag = if first {
                    first = false;
                    " \x1b[1;32m[current]\x1b[0m"
                } else {
                    ""
                };
                println!("{color}  │   v{}: {}  \x1b[2m[{} words]\x1b[0m{}{color}",
                    version.version_number, version.content, wc, tag);
            }
        }

        let tags = note.tags.join(", #");
        println!("{color}  │   #{tags}  |  color: {}  |  links: {:?}",
            note.color.as_str(), note.links);
        println!("{color}  └───────────────────────────{reset}\n");
    }
}
//     pub fn show_all_notes(&self) {
//         println!("\n\x1b[1;34m╔══════════════════════════════╗");
//         println!("║        📒 MY NOTES           ║");
//         println!("╚══════════════════════════════╝\x1b[0m\n");

//         if self.notes.is_empty() {
//             println!("\x1b[33m  No notes found.\x1b[0m\n");
//             return;
//         }

//         // Pinned first
//         let mut sorted: Vec<&Note> = self.notes.iter().collect();
//         sorted.sort_by(|a, b| b.pinned.cmp(&a.pinned));

//         for note in sorted {
//             let color = note.color.to_ansi();
//             let reset = "\x1b[0m";
//             let pin   = if note.pinned { "★ " } else { "  " };
//             let due   = if note.is_reminder_due() { " 🔔" } else { "" };

//             println!("{color}  ┌─ {pin}Note #{} ─ {}{due}", note.id, note.title);

//             if note.versions.is_empty() {
//                 println!("{color}  │   (no versions yet){reset}");
//             } else {
//                 let latest_vn = note.versions.len() as u32;
// for version in &note.versions {
//     let wc = version.word_count();
//     let tag = if version.version_number == latest_vn {
//         " \x1b[1;32m[current]\x1b[0m"
//     } else {
//         ""
//     };
//     println!("{color}  │   v{}: {}  \x1b[2m[{} words]\x1b[0m{}{color}",
//         version.version_number, version.content, wc, tag);
// }
//             }

//             let tags = note.tags.join(", #");
//             println!("{color}  │   #{tags}  |  color: {}  |  links: {:?}",
//                 note.color.as_str(), note.links);
//             println!("{color}  └───────────────────────────{reset}\n");
//         }
//     }

    // ── Delete with undo support ─────────────────────────────────────────────

    pub fn delete_note_by_id(&mut self, id: u32) -> bool {
        if let Some(pos) = self.notes.iter().position(|n| n.id == id) {
            let note = self.notes.remove(pos);
            self.trash.push(note);
            true
        } else {
            false
        }
    }

    pub fn undo_delete(&mut self) -> Option<u32> {
        if let Some(note) = self.trash.pop() {
            let id = note.id;
            self.notes.push(note);
            // Keep sorted by id
            self.notes.sort_by_key(|n| n.id);
            Some(id)
        } else {
            None
        }
    }

    // ── Edit ─────────────────────────────────────────────────────────────────
pub fn edit_note_by_id(&mut self, id: u32) {
    use std::io::{self, Write};

    if let Some(note) = self.get_note_by_id(id) {
        println!("\x1b[1;33m\n✏️  Editing Note #{}: {}\x1b[0m", note.id, note.title);

        if let Some(last_version) = note.versions.last() {
            println!("\x1b[36m  Current content (v{}):\x1b[0m", last_version.version_number);
            println!("\x1b[2m{}\x1b[0m", last_version.content);
        }

        println!("\x1b[1;96m  Enter content (multiple lines supported).\x1b[0m");
        println!("\x1b[2m  Type \x1b[0m\x1b[1;31m;;\x1b[0m\x1b[2m on a new line and press Enter to finish.\x1b[0m\n");

        let mut lines: Vec<String> = Vec::new();
        loop {
            print!("\x1b[1;93m  │\x1b[0m ");
            io::stdout().flush().expect("flush");

            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("read");
            let trimmed = line.trim_end_matches('\n')
                              .trim_end_matches('\r')
                              .to_string();

            if trimmed == ";;" {
                break;
            }
            lines.push(trimmed);
        }

        let new_content = lines.join("\n");
        if new_content.is_empty() {
            println!("\x1b[1;31m  ⚠ Empty content — version not saved.\x1b[0m");
            return;
        }

        note.add_version(new_content);

        if let Some(last_version) = note.versions.last() {
            println!("\x1b[1;32m  ✅ Version {} saved! ({} lines, {} words)\x1b[0m",
                last_version.version_number,
                last_version.content.lines().count(),
                last_version.word_count());
        }
    } else {
        println!("\x1b[1;31m  ❌ Note with ID {} not found.\x1b[0m", id);
    }
}
    // pub fn edit_note_by_id(&mut self, id: u32) {
    //     use std::io::{self, Write};

    //     if let Some(note) = self.get_note_by_id(id) {
    //         println!("\x1b[1;33m\n✏️  Editing Note #{}: {}\x1b[0m", note.id, note.title);

    //         if let Some(last_version) = note.versions.last() {
    //             println!("\x1b[36m  Current content (v{}): {}\x1b[0m",
    //                 last_version.version_number, last_version.content);
    //         }

    //         print!("\x1b[1;37m  Enter new content: \x1b[0m");
    //         io::stdout().flush().expect("flush");

    //         let mut new_content = String::new();
    //         io::stdin().read_line(&mut new_content).expect("read");
    //         note.add_version(new_content.trim().to_string());

    //         if let Some(last_version) = note.versions.last() {
    //             println!("\x1b[1;32m  ✅ Version {} saved!\x1b[0m", last_version.version_number);
    //         }
    //     } else {
    //         println!("\x1b[1;31m  ❌ Note with ID {} not found.\x1b[0m", id);
    //     }
    // }

    // ── Color ─────────────────────────────────────────────────────────────────

    pub fn set_color(&mut self, id: u32, color: NoteColor) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            note.color = color;
            true
        } else {
            false
        }
    }

    // ── Tags ──────────────────────────────────────────────────────────────────

    pub fn bulk_add_tag(&mut self, tag: &str) {
        for note in &mut self.notes {
            if !note.tags.contains(&tag.to_string()) {
                note.tags.push(tag.to_string());
            }
        }
    }

    pub fn bulk_remove_tag(&mut self, tag: &str) {
        for note in &mut self.notes {
            note.tags.retain(|t| t != tag);
        }
    }

    pub fn add_tag_to_note(&mut self, id: u32, tag: &str) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            if !note.tags.contains(&tag.to_string()) {
                note.tags.push(tag.to_string());
            }
            true
        } else { false }
    }

    pub fn remove_tag_from_note(&mut self, id: u32, tag: &str) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            note.tags.retain(|t| t != tag);
            true
        } else { false }
    }
pub fn edit_version_of_note(&mut self, note_id: u32, version_number: u32) -> bool {
    use std::io::{self, Write};

    let existing = if let Some(note) = self.get_note_by_id(note_id) {
        if let Some(v) = note.versions.iter().find(|v| v.version_number == version_number) {
            v.content.clone()
        } else {
            println!("\x1b[1;31m  ❌ Version {} not found.\x1b[0m", version_number);
            return false;
        }
    } else {
        println!("\x1b[1;31m  ❌ Note with ID {} not found.\x1b[0m", note_id);
        return false;
    };

    println!("\x1b[1;36m  Current content (v{}):\x1b[0m", version_number);
    for line in existing.lines() {
        println!("\x1b[2m  │ {}\x1b[0m", line);
    }

    println!("\n\x1b[1;33m  What do you want to do?\x1b[0m");
    println!("  1) Append to existing content");
    println!("  2) Replace with new content");
    print!("\x1b[1;96m  Choice\x1b[0m \x1b[1;93m>\x1b[0m ");
    io::stdout().flush().expect("flush");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("read");
    let choice = choice.trim().to_string();

    println!("\x1b[2m  Type ;; on a new line to finish.\x1b[0m\n");

    let mut new_lines: Vec<String> = Vec::new();
    loop {
        print!("\x1b[1;93m  │\x1b[0m ");
        io::stdout().flush().expect("flush");
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("read");
        let trimmed = line.trim_end_matches('\n')
                          .trim_end_matches('\r')
                          .to_string();
        if trimmed == ";;" { break; }
        new_lines.push(trimmed);
    }

    if new_lines.is_empty() {
        println!("\x1b[1;31m  ⚠ Nothing entered — version not updated.\x1b[0m");
        return false;
    }

    let final_content = match choice.as_str() {
        "1" => format!("{}\n{}", existing, new_lines.join("\n")),
        _   => new_lines.join("\n"),
    };

    if let Some(note) = self.get_note_by_id(note_id) {
        note.edit_version(version_number, final_content);
        println!("\x1b[1;32m  ✅ Version {} updated!\x1b[0m", version_number);
        true
    } else {
        false
    }
}
    // ── Password ─────────────────────────────────────────────────────────────

    pub fn set_password(&mut self, id: u32, password: String) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            note.password = Some(password);
            true
        } else { false }
    }

    pub fn remove_password(&mut self, id: u32) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            note.password = None;
            true
        } else { false }
    }

    pub fn check_password(&self, id: u32, attempt: &str) -> bool {
        if let Some(note) = self.get_note_by_id_ref(id) {
            match &note.password {
                Some(p) => p == attempt,
                None    => true,
            }
        } else { false }
    }
   pub fn delete_version(&mut self, note_id: u32, version_number: u32) -> bool {
    if let Some(note) = self.get_note_by_id(note_id) {
        if note.versions.len() <= 1 {
            println!("\x1b[1;31m  ❌ Cannot delete the only version. Delete the note instead.\x1b[0m");
            return false;
        }
        let before = note.versions.len();
        note.versions.retain(|v| v.version_number != version_number);
        if note.versions.len() < before {
            // Renumber versions so they stay sequential
            for (i, v) in note.versions.iter_mut().enumerate() {
                v.version_number = i as u32 + 1;
            }
            note.updated_at = crate::note::now_timestamp();
            return true;
        }
    }
    false
}
    // ── Reminder ─────────────────────────────────────────────────────────────

    pub fn set_reminder(&mut self, id: u32, ts: u64) -> bool {
        if let Some(note) = self.get_note_by_id(id) {
            note.reminder = Some(ts);
            true
        } else { false }
    }

    pub fn due_reminders(&self) -> Vec<&Note> {
        self.notes.iter().filter(|n| n.is_reminder_due()).collect()
    }
  pub fn clear_due_reminders(&mut self) {
    for note in &mut self.notes {
        if note.is_reminder_due() {
            note.reminder = None;
        }
    }
}
    // ── Note linking ─────────────────────────────────────────────────────────

    pub fn link_notes(&mut self, from_id: u32, to_id: u32) -> bool {
        // Verify both exist
        let exists_to = self.notes.iter().any(|n| n.id == to_id);
        if !exists_to { return false; }
        if let Some(note) = self.get_note_by_id(from_id) {
            if !note.links.contains(&to_id) {
                note.links.push(to_id);
            }
            true
        } else { false }
    }

    pub fn unlink_notes(&mut self, from_id: u32, to_id: u32) -> bool {
        if let Some(note) = self.get_note_by_id(from_id) {
            note.links.retain(|&id| id != to_id);
            true
        } else { false }
    }
    pub fn restore_from_trash(&mut self, id: u32) -> bool {
    if let Some(pos) = self.trash.iter().position(|n| n.id == id) {
        let note = self.trash.remove(pos);
        self.notes.push(note);
        self.notes.sort_by_key(|n| n.id);
        true
    } else {
        false
    }
}

pub fn restore_all_from_trash(&mut self) {
    let mut all = self.trash.drain(..).collect::<Vec<_>>();
    self.notes.append(&mut all);
    self.notes.sort_by_key(|n| n.id);
}

pub fn delete_from_trash(&mut self, id: u32) -> bool {
    if let Some(pos) = self.trash.iter().position(|n| n.id == id) {
        self.trash.remove(pos);
        true
    } else {
        false
    }
}

pub fn empty_trash(&mut self) {
    self.trash.clear();
}
}