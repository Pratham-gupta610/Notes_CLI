// ui/mod.rs

use std::io::{self, Write};
use crate::manager::NoteManager;
use crate::note::{NoteColor, format_timestamp, now_timestamp};

// ── Prompt colors ─────────────────────────────────────────────────────────────
// All input prompts use bright cyan so they stand out clearly on dark terminals
const P:  &str = "\x1b[1;96m";   // bright cyan  — prompt label
const PI: &str = "\x1b[1;93m";   // bright yellow — prompt arrow >
const R:  &str = "\x1b[0m";      // reset

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();
}

/// All user-facing prompts go through here — colored cyan label + yellow arrow
fn prompt(msg: &str) -> String {
    print!("{P}{msg}{R} {PI}>{R} ");
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn pause() {
    print!("\n  \x1b[2mPress Enter to continue…{R}");
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
}

fn read_id(label: &str) -> Option<u32> {
    prompt(label).parse::<u32>().ok()
}

pub fn print_logo() {
    // Single print — no double logo
    println!("\x1b[1;35m");
    println!("  ███╗   ██╗ ██████╗ ████████╗███████╗███████╗");
    println!("  ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝");
    println!("  ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗");
    println!("  ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║");
    println!("  ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║");
    println!("  ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝");
    println!("{R}");
}
fn menu_trash(nm: &mut NoteManager, file_path: &str) {
    loop {
        println!("\n\x1b[1;31m  ╔══════════════════════════════════════╗");
        println!("  ║           🗑  TRASH BIN               ║");
        println!("  ╚══════════════════════════════════════╝\x1b[0m\n");

        if nm.trash.is_empty() {
            println!("  \x1b[2m  Trash is empty.\x1b[0m\n");
        } else {
            println!("  \x1b[2m  {} note(s) in trash:\x1b[0m\n", nm.trash.len());
            for note in &nm.trash {
                println!("  \x1b[1;31m  #{}\x1b[0m \x1b[1;37m{}\x1b[0m  \x1b[2m[{}v, {}w, tags: #{}]\x1b[0m",
                    note.id,
                    note.title,
                    note.versions.len(),
                    note.total_words(),
                    note.tags.join(", #"));
            }
        }

        println!("\n  \x1b[2m1) Restore by ID     2) Restore All");
        println!("  3) Delete by ID      4) Empty Trash");
        println!("  0) Back\x1b[0m\n");

        let ch = prompt("  Option");
        match ch.as_str() {
            "1" => {
                if nm.trash.is_empty() {
                    println!("\x1b[1;31m  Trash is empty.\x1b[0m");
                    continue;
                }
                match read_id("  Note ID to restore") {
                    Some(id) => {
                        if nm.restore_from_trash(id) {
                            autosave_if(nm, file_path);
                            println!("\x1b[1;32m  ✅ Note #{id} restored!\x1b[0m");
                        } else {
                            println!("\x1b[1;31m  ❌ Note #{id} not found in trash.\x1b[0m");
                        }
                    }
                    None => println!("\x1b[1;31m  ❌ Invalid ID.\x1b[0m"),
                }
            }
            "2" => {
                if nm.trash.is_empty() {
                    println!("\x1b[1;31m  Trash is empty.\x1b[0m");
                    continue;
                }
                let count = nm.trash.len();
                nm.restore_all_from_trash();
                autosave_if(nm, file_path);
                println!("\x1b[1;32m  ✅ Restored {count} note(s)!\x1b[0m");
            }
            "3" => {
                if nm.trash.is_empty() {
                    println!("\x1b[1;31m  Trash is empty.\x1b[0m");
                    continue;
                }
                match read_id("  Note ID to permanently delete") {
                    Some(id) => {
                        let confirm = prompt("  Are you sure? Type 'yes'");
                        if confirm == "yes" {
                            if nm.delete_from_trash(id) {
                                autosave_if(nm, file_path);
                                println!("\x1b[1;32m  ✅ Note #{id} permanently deleted.\x1b[0m");
                            } else {
                                println!("\x1b[1;31m  ❌ Note #{id} not found in trash.\x1b[0m");
                            }
                        }
                    }
                    None => println!("\x1b[1;31m  ❌ Invalid ID.\x1b[0m"),
                }
            }
            "4" => {
                if nm.trash.is_empty() {
                    println!("\x1b[1;31m  Trash is empty.\x1b[0m");
                    continue;
                }
                let confirm = prompt("  Empty trash permanently? Type 'yes'");
                if confirm == "yes" {
                    let count = nm.trash.len();
                    nm.empty_trash();
                    autosave_if(nm, file_path);
                    println!("\x1b[1;32m  ✅ Trash emptied ({count} notes permanently deleted).\x1b[0m");
                }
            }
            "0" | "" => break,
            _ => println!("\x1b[1;31m  Unknown option.\x1b[0m"),
        }
    }
}
// ── Main menu ─────────────────────────────────────────────────────────────────

pub fn main_menu(nm: &mut NoteManager, file_path: &str) {
    loop {
        // Due reminders banner
      let due = nm.due_reminders();
if !due.is_empty() {
    // std::process::Command::new("powershell")
    //     .args(["-c", "[console]::beep(800,300); [console]::beep(800,300); [console]::beep(800,300)"])
    //     .output()
    //     .ok();
std::process::Command::new("mshta")
    .arg("vbscript:close(CreateObject(\"SAPI.SpVoice\").Speak(\"Reminder due\"))")
    .output()
    .ok();
    println!("\x1b[1;33m╔══════════════════════════════════════╗");
    println!("║  🔔  REMINDER DUE!                   ║");
    println!("╚══════════════════════════════════════╝\x1b[0m");
    for n in &due {
        println!("     \x1b[1;33m• #{} {}\x1b[0m", n.id, n.title);
    }
    println!();

    // Clear all fired reminders
    nm.clear_due_reminders();
    autosave_if(nm, file_path);
}

        let total_versions: usize = nm.notes.iter().map(|n| n.versions.len()).sum();
        let pinned = nm.notes.iter().filter(|n| n.pinned).count();
        let autosave_str = if nm.autosave {
            "\x1b[1;32mON\x1b[0m"
        } else {
            "\x1b[1;31mOFF\x1b[0m"
        };

        println!("\x1b[2m  {} notes · {} pinned · {} versions · autosave {}\x1b[0m\n",
            nm.notes.len(), pinned, total_versions, autosave_str);

        println!("\x1b[1;37m  Main Menu\x1b[0m");
        println!("\x1b[1;36m  ┌─ MAIN MENU ─────────────────────────────────────────┐\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  1.  Create Note                         \x1b[2m[new]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  2.  Find / Open Note                     \x1b[2m[→]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  3.  Show All Notes");
        println!("  \x1b[1;36m│\x1b[0m  4.  Search Notes                         \x1b[2m[→]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  5.  Favorites / Pinned Notes             \x1b[2m[Imp]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  6.  Statistics Dashboard");
        println!("  \x1b[1;36m│\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  7.  Delete All Notes                    \x1b[2m[!]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  8.  Save Notes");
        println!("  \x1b[1;36m│\x1b[0m  9.  Load Notes");
        println!("  \x1b[1;36m│\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  10. Settings");
        println!("  \x1b[1;36m│\x1b[0m  11. Help");
        println!("  \x1b[1;36m│\x1b[0m  12. Exit                              \x1b[2m[quit]\x1b[0m");
        println!("  \x1b[1;36m│\x1b[0m  13. Trash Bin                        \x1b[2m[🗑 ]\x1b[0m");
        println!("  \x1b[1;36m└─────────────────────────────────────────────────────┘\x1b[0m\n");

        let choice = prompt("  Choice");

        match choice.as_str() {
            "1"  => menu_create_note(nm, file_path),
            "2"  => menu_open_note(nm, file_path),
            "3"  => { nm.show_all_notes(); pause(); }
            "4"  => menu_search(nm),
            "5"  => menu_pinned(nm),
            "6"  => { crate::stats::print_dashboard(nm); pause(); }
            "7"  => menu_delete_all(nm, file_path),
            "8"  => menu_save(nm, file_path),
            "9"  => menu_load(nm, file_path),
            "10" => menu_settings(nm, file_path),
            "11" => menu_help(),
            "12" | "q" | "quit" | "exit" => {
                if nm.autosave {
                    let _ = crate::storage::save_notes_to_file(nm, file_path);
                }
                println!("\n  \x1b[1;35m  Goodbye! 👋\x1b[0m\n");
                break;
            }
            "13" => menu_trash(nm, file_path),
            _ => println!("\x1b[1;31m  Unknown option.\x1b[0m"),
        }
    }
}

// ── Create ────────────────────────────────────────────────────────────────────

fn menu_create_note(nm: &mut NoteManager, file_path: &str) {
    println!("\n\x1b[1;36m  ─── Create Note ───────────────────────────\x1b[0m");
    println!("  \x1b[2m1) Blank  2) Meeting  3) To-Do  4) Journal  5) Custom\x1b[0m");
    let t = prompt("  Template (1-5, default 1)");

    let template: Option<&str> = match t.as_str() {
        "2" => Some("## Attendees\n\n## Agenda\n\n## Action Items\n\n## Notes\n"),
        "3" => Some("## To-Do\n\n- [ ] Item 1\n- [ ] Item 2\n- [ ] Item 3\n"),
        "4" => Some("## Date\n\n## How I'm feeling\n\n## Today's highlights\n\n## Reflections\n"),
        "5" => {
            let tpl = prompt("  Enter your template text");
            let title = prompt("  Note title");
            nm.create_note_from_template(title, &tpl);
            autosave_if(nm, file_path);
            println!("\x1b[1;32m  ✅ Note created from custom template!\x1b[0m");
            return;
        }
        _ => None,
    };

    let title = prompt("  Title");
    if let Some(tpl) = template {
        nm.create_note_from_template(title, tpl);
    } else {
        nm.create_note(title);
    }
    autosave_if(nm, file_path);
    println!("\x1b[1;32m  ✅ Note created!\x1b[0m");
}

// ── Open / Note detail menu ───────────────────────────────────────────────────

fn menu_open_note(nm: &mut NoteManager, file_path: &str) {
    nm.show_all_notes();
    let id = match read_id("  Enter Note ID") { Some(id) => id, None => return };

    // Password check
    {
        let locked = nm.get_note_by_id_ref(id).map(|n| n.password.is_some()).unwrap_or(false);
        if locked {
            let attempt = prompt("  🔒 Password");
            if !nm.check_password(id, &attempt) {
                println!("\x1b[1;31m  ❌ Wrong password.\x1b[0m");
                return;
            }
        }
    }

    loop {
        let info = {
            if let Some(note) = nm.get_note_by_id_ref(id) {
                Some((
                    note.title.clone(),
                    note.versions.len(),
                    note.latest_word_count(),
                    note.latest_char_count(),
                    note.pinned,
                    note.color.as_str().to_string(),
                    note.tags.join(", #"),
                    note.links.iter().map(|l| format!("#{}", l)).collect::<Vec<_>>().join(", "),
                    format_timestamp(note.created_at),
                    format_timestamp(note.updated_at),
                    note.reminder.map(|r| format_timestamp(r)).unwrap_or_else(|| "none".to_string()),
                ))
            } else { None }
        };

        let (title, v_count, wc, cc, pinned, color_str, tags_str, links_str, created, updated, reminder_str)
            = match info { Some(x) => x, None => { println!("\x1b[1;31m  ❌ Note not found.\x1b[0m"); return; } };

        let pin_str = if pinned { "★ pinned" } else { "not pinned" };

        println!("\n\x1b[1;36m  ╔═══ #{id} — {title} ═══\x1b[0m");
        println!("  \x1b[2mTags: #{tags_str}  |  Color: {color_str}  |  {pin_str}\x1b[0m");
        println!("  \x1b[2mVersions: {v_count}  |  Latest: {wc} words, {cc} chars\x1b[0m");
        println!("  \x1b[2mCreated:  {created}\x1b[0m");
        println!("  \x1b[2mUpdated:  {updated}\x1b[0m");
        println!("  \x1b[2mLinks: {links_str}  |  Reminder: {reminder_str}\x1b[0m");
        println!("  \x1b[1;36m╠══ Versions ══════════════════════════\x1b[0m");

        // Show all versions with individual timestamps
        if let Some(note) = nm.get_note_by_id_ref(id) {
            if note.versions.is_empty() {
                println!("  \x1b[2m  (no versions yet)\x1b[0m");
            } else {
                for v in &note.versions {
                    println!("{}", v.display_line(false));
                }
            }
        }
        println!("  \x1b[1;36m╚══════════════════════════════════════\x1b[0m");

       println!("  \x1b[2ma) Edit      b) Pin/Unpin    c) Set Color     d) Tags");
println!("  e) Diff      f) Export       g) Import .md    h) Password");
println!("  i) Reminder  j) Link Notes   k) Delete Note   ");
println!("  l) View Linked               m) Delete Version");
println!("  n) Edit Version (advanced)\x1b[0m");
println!("  0) Back\x1b[0m\n");
        let ch = prompt("  Option");
        match ch.as_str() {
            "a" => {
                nm.edit_note_by_id(id);
                autosave_if(nm, file_path);
            }
            "b" => {
                if let Some(note) = nm.get_note_by_id(id) {
                    note.pinned = !note.pinned;
                    println!("\x1b[1;32m  ✅ Note {}.\x1b[0m", if note.pinned { "pinned ★" } else { "unpinned" });
                }
                autosave_if(nm, file_path);
            }
            "c" => {
                println!("  {}", NoteColor::menu_str());
                let n = prompt("  Choose color");
                nm.set_color(id, NoteColor::from_menu(&n));
                autosave_if(nm, file_path);
                println!("\x1b[1;32m  ✅ Color updated.\x1b[0m");
            }
            "d" => menu_tags_for_note(nm, id, file_path),
            "e" => menu_diff(nm, id),
            "f" => menu_export(nm, id),
            "g" => menu_import_md(nm, id, file_path),
            "h" => menu_password(nm, id),
            "i" => menu_reminder(nm, id, file_path),
            "j" => menu_link(nm, id, file_path),
            "k" => {
                let confirm = prompt("  Delete this note? Type 'yes' to confirm");
                if confirm == "yes" {
                    nm.delete_note_by_id(id);
                    autosave_if(nm, file_path);
                    println!("\x1b[1;32m  ✅ Deleted (Undo Delete restores it).\x1b[0m");
                    return;
                }
            }
            // "l" => {
            //     match nm.undo_delete() {
            //         Some(rid) => println!("\x1b[1;32m  ✅ Note #{rid} restored!\x1b[0m"),
            //         None      => println!("\x1b[1;31m  Nothing to undo.\x1b[0m"),
            //     }
            //     autosave_if(nm, file_path);
            // }
            "l" => menu_view_linked(nm, id),
            "m" => {
                let vnum: u32 = match prompt("  Version number to delete").parse() {
                    Ok(n) => n,
                    Err(_) => { println!("\x1b[1;31m  ❌ Invalid number.\x1b[0m"); continue; }
                };
                if nm.delete_version(id, vnum) {
                    println!("\x1b[1;32m  ✅ Version {vnum} deleted and versions renumbered.\x1b[0m");
                    autosave_if(nm, file_path);
                } else {
                    println!("\x1b[1;31m  ❌ Version {vnum} not found.\x1b[0m");
                }
            }
            "n" => {
    let vnum: u32 = match prompt("  Version number to edit").parse() {
        Ok(n) => n,
        Err(_) => { println!("\x1b[1;31m  ❌ Invalid number.\x1b[0m"); continue; }
    };
    if nm.edit_version_of_note(id, vnum) {
        autosave_if(nm, file_path);
    }
}
            "0" | "" => break,
            _ => println!("\x1b[1;31m  Unknown option.\x1b[0m"),
        }
    }
}

// ── Tags ──────────────────────────────────────────────────────────────────────

fn menu_tags_for_note(nm: &mut NoteManager, id: u32, file_path: &str) {
    println!("\n  \x1b[1;33mTag Editor\x1b[0m");
    println!("  1) Add tag   2) Remove tag   3) Bulk add to ALL   4) Bulk remove from ALL");
    let ch = prompt("  Option");
    match ch.as_str() {
        "1" => { let tag = prompt("  Tag name"); nm.add_tag_to_note(id, &tag); autosave_if(nm, file_path); println!("\x1b[1;32m  ✅ Tag added.\x1b[0m"); }
        "2" => { let tag = prompt("  Tag to remove"); nm.remove_tag_from_note(id, &tag); autosave_if(nm, file_path); println!("\x1b[1;32m  ✅ Tag removed.\x1b[0m"); }
        "3" => { let tag = prompt("  Tag to add to ALL notes"); nm.bulk_add_tag(&tag); autosave_if(nm, file_path); println!("\x1b[1;32m  ✅ Tag added to all notes.\x1b[0m"); }
        "4" => { let tag = prompt("  Tag to remove from ALL notes"); nm.bulk_remove_tag(&tag); autosave_if(nm, file_path); println!("\x1b[1;32m  ✅ Tag removed from all notes.\x1b[0m"); }
        _ => {}
    }
}

// ── Diff ──────────────────────────────────────────────────────────────────────

fn menu_diff(nm: &NoteManager, id: u32) {
    if let Some(note) = nm.get_note_by_id_ref(id) {
        if note.versions.len() < 2 {
            println!("\x1b[1;31m  Need at least 2 versions to diff.\x1b[0m");
            return;
        }
        let v1: u32 = prompt("  Version A").parse().unwrap_or(1);
        let v2: u32 = prompt("  Version B").parse().unwrap_or(2);
        match note.diff(v1, v2) {
            Some(d) => println!("{}", d),
            None    => println!("\x1b[1;31m  Version(s) not found.\x1b[0m"),
        }
        pause();
    }
}

// ── Export ────────────────────────────────────────────────────────────────────

fn menu_export(nm: &NoteManager, id: u32) {
    if let Some(note) = nm.get_note_by_id_ref(id) {
        println!("  1) Markdown (.md)   2) Plain text (.txt)");
        let ch = prompt("  Format");
        let path = match ch.as_str() {
            "1" => { let p = format!("note_{}.md", note.id); crate::export::export_markdown(note, &p).ok(); p }
            _   => { let p = format!("note_{}.txt", note.id); crate::export::export_plain_text(note, &p).ok(); p }
        };
        println!("\x1b[1;32m  ✅ Exported to {path}\x1b[0m");
    }
}

// ── Import .md ────────────────────────────────────────────────────────────────

fn menu_import_md(nm: &mut NoteManager, id: u32, file_path: &str) {
    let path = prompt("  Path to .md/.txt file");
    match crate::export::import_markdown(&path) {
        Ok(content) => {
            if let Some(note) = nm.get_note_by_id(id) { note.add_version(content); }
            autosave_if(nm, file_path);
            println!("\x1b[1;32m  ✅ Imported as new version!\x1b[0m");
        }
        Err(e) => println!("\x1b[1;31m  ❌ Error: {e}\x1b[0m"),
    }
}

// ── Password ──────────────────────────────────────────────────────────────────

fn menu_password(nm: &mut NoteManager, id: u32) {
    println!("  1) Set password   2) Remove password");
    let ch = prompt("  Option");
    match ch.as_str() {
        "1" => { let pw = prompt("  New password"); nm.set_password(id, pw); println!("\x1b[1;32m  ✅ Password set.\x1b[0m"); }
        "2" => { nm.remove_password(id); println!("\x1b[1;32m  ✅ Password removed.\x1b[0m"); }
        _ => {}
    }
}

// ── Reminder ─────────────────────────────────────────────────────────────────

fn menu_reminder(nm: &mut NoteManager, id: u32, file_path: &str) {
    println!("  Enter seconds from now (e.g. 3600 = 1 hr, 86400 = 1 day)");
    let secs: u64 = prompt("  Seconds from now").parse().unwrap_or(0);
    if secs > 0 {
        let ts = now_timestamp() + secs;
        nm.set_reminder(id, ts);
        autosave_if(nm, file_path);
        println!("\x1b[1;32m  ✅ Reminder set for {}\x1b[0m", format_timestamp(ts));
    }
}

// ── Note linking ──────────────────────────────────────────────────────────────

fn menu_link(nm: &mut NoteManager, from_id: u32, file_path: &str) {
    println!("  1) Link to another note   2) Unlink");
    let ch = prompt("  Option");
    match ch.as_str() {
        "1" => {
            let to_id = match read_id("  Link to Note ID") { Some(x) => x, None => return };
            if nm.link_notes(from_id, to_id) {
                autosave_if(nm, file_path);
                println!("\x1b[1;32m  ✅ Linked #{from_id} → #{to_id}\x1b[0m");
            } else {
                println!("\x1b[1;31m  ❌ Target note not found.\x1b[0m");
            }
        }
        "2" => {
            let to_id = match read_id("  Unlink Note ID") { Some(x) => x, None => return };
            nm.unlink_notes(from_id, to_id);
            autosave_if(nm, file_path);
            println!("\x1b[1;32m  ✅ Unlinked.\x1b[0m");
        }
        _ => {}
    }
}

fn menu_view_linked(nm: &NoteManager, id: u32) {
    if let Some(note) = nm.get_note_by_id_ref(id) {
        if note.links.is_empty() { println!("  No linked notes."); return; }
        println!("\n  \x1b[1;36mLinked notes:\x1b[0m");
        for &lid in &note.links {
            if let Some(linked) = nm.get_note_by_id_ref(lid) {
                println!("  → #{} {} \x1b[2m({} versions, {} words)\x1b[0m",
                    linked.id, linked.title, linked.versions.len(), linked.total_words());
            } else {
                println!("  → #{lid} \x1b[2m(deleted)\x1b[0m");
            }
        }
        pause();
    }
}

// ── Search ────────────────────────────────────────────────────────────────────

fn menu_search(nm: &NoteManager) {
    println!("\n\x1b[1;36m  ─── Search ─────────────────────────────\x1b[0m");
    println!("  1) Full-text   2) By tag   3) By title   4) By content");
    let mode  = prompt("  Mode");
    let query = prompt("  Query").to_lowercase();

    let indices: Vec<usize> = match mode.as_str() {
        "2" => crate::search::search_by_tag(&nm.notes, &query),
        "3" => crate::search::search_by_title(&nm.notes, &query),
        "4" => crate::search::search_by_content(&nm.notes, &query).into_iter().map(|(i,_)| i).collect(),
        _   => crate::search::search_full_text(&nm.notes, &query),
    };

    if indices.is_empty() {
        println!("\x1b[1;31m  No results for \"{query}\"\x1b[0m");
    } else {
        println!("\x1b[1;32m  {} result(s):\x1b[0m", indices.len());
        for i in indices {
            if let Some(note) = nm.notes.get(i) {
                println!("  {}#{} {}\x1b[0m \x1b[2m[{}v, {}w]\x1b[0m",
                    note.color.to_ansi(), note.id, note.title,
                    note.versions.len(), note.total_words());
            }
        }
    }
    pause();
}

// ── Pinned ────────────────────────────────────────────────────────────────────

fn menu_pinned(nm: &NoteManager) {
    println!("\n\x1b[1;33m  ─── Pinned Notes ────────────────────────\x1b[0m");
    let pinned: Vec<&crate::note::Note> = nm.notes.iter().filter(|n| n.pinned).collect();
    if pinned.is_empty() {
        println!("  No pinned notes.");
    } else {
        for note in pinned {
            println!("  {}★.red #{} {}\x1b[0m \x1b[2m#{}\x1b[0m",
                note.color.to_ansi(), note.id, note.title, note.tags.join(", #"));
        }
    }
    pause();
}

// ── Delete all ────────────────────────────────────────────────────────────────

fn menu_delete_all(nm: &mut NoteManager, file_path: &str) {
    let confirm = prompt("  \x1b[1;31mType DELETE to confirm wiping all notes");
    if confirm == "DELETE" {
        let ids: Vec<u32> = nm.notes.iter().map(|n| n.id).collect();
        for id in ids { nm.delete_note_by_id(id); }
        autosave_if(nm, file_path);
        println!("\x1b[1;32m  ✅ All notes deleted (Undo Delete restores one at a time).\x1b[0m");
    }
}

// ── Save / Load ───────────────────────────────────────────────────────────────

fn menu_save(nm: &NoteManager, file_path: &str) {
    match crate::storage::save_notes_to_file(nm, file_path) {
        Ok(_)  => println!("\x1b[1;32m  ✅ Saved to {file_path}\x1b[0m"),
        Err(e) => println!("\x1b[1;31m  ❌ Save error: {e}\x1b[0m"),
    }
}

fn menu_load(nm: &mut NoteManager, file_path: &str) {
    match crate::storage::load_notes_from_file(file_path) {
        Ok(loaded) => { *nm = loaded; println!("\x1b[1;32m  ✅ Loaded {} notes.\x1b[0m", nm.notes.len()); }
        Err(e)     => println!("\x1b[1;31m  ❌ Load error: {e}\x1b[0m"),
    }
}

// ── Settings ──────────────────────────────────────────────────────────────────

fn menu_settings(nm: &mut NoteManager, file_path: &str) {
    println!("\n\x1b[1;36m  ─── Settings ───────────────────────────\x1b[0m");
    let state = if nm.autosave { "\x1b[1;32mON\x1b[0m" } else { "\x1b[1;31mOFF\x1b[0m" };
    println!("  1) Toggle autosave (currently {state})");
    println!("  2) Clear screen");
    let ch = prompt("  Option");
    match ch.as_str() {
        "1" => {
            nm.autosave = !nm.autosave;
            println!("\x1b[1;32m  ✅ Autosave {}.\x1b[0m", if nm.autosave { "enabled" } else { "disabled" });
        }
        "2" => clear_screen(),
        _ => {}
    }
    let _ = file_path;
}

// ── Help ─────────────────────────────────────────────────────────────────────

fn menu_help() {
    println!("\n\x1b[1;36m  ─── Help ───────────────────────────────\x1b[0m");
    let items = [
        "Every edit creates a new version with its own IST timestamp",
        "Diff: compare any two versions line-by-line",
        "Colors: assign a color label per note",
        "Timestamps: note created/updated + per-version created time",
        "Word & char counts shown per version",
        "Tags: multi-tag support, bulk-edit across all notes",
        "Pinning: star important notes, shown first",
        "Undo Delete: deleted notes go to trash, restorable",
        "Delete Version: remove a specific version (renumbers rest)",
        "Templates: Meeting, To-Do, Journal, or custom",
        "Password protect individual notes",
        "Reminders: set a time (seconds from now), see 🔔 when due",
        "Note linking: connect related notes by ID",
        "Export: .md or .txt  |  Import: .md/.txt file as version",
        "Full-text search across title, tags, and content",
        "Statistics dashboard with word counts and visual bars",
    ];
    for item in &items {
        println!("  \x1b[1;36m•\x1b[0m {item}");
    }
    pause();
}

// ── Autosave helper ───────────────────────────────────────────────────────────

fn autosave_if(nm: &NoteManager, file_path: &str) {
    if nm.autosave { let _ = crate::storage::save_notes_to_file(nm, file_path); }
}

#[allow(dead_code)]
pub fn box_row(width: usize, content: &str, color: &str) -> String {
    format!("{color}  │ {content:<width$} │\x1b[0m")
}

#[allow(dead_code)]
pub fn box_separator(width: usize) -> String {
    format!("  ├{}┤", "─".repeat(width + 2))
}