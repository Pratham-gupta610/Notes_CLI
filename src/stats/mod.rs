// stats/mod.rs

use crate::manager::NoteManager;
use crate::note::format_timestamp;

pub fn print_dashboard(nm: &NoteManager) {
    let total_notes    = nm.notes.len();
    let total_versions: usize = nm.notes.iter().map(|n| n.versions.len()).sum();
    let pinned         = nm.notes.iter().filter(|n| n.pinned).count();
    let total_words: usize = nm.notes.iter().map(|n| n.total_words()).sum();
    let total_chars: usize = nm.notes.iter().map(|n| n.latest_char_count()).sum();
    let avg_versions   = if total_notes > 0 { total_versions as f64 / total_notes as f64 } else { 0.0 };
    let due_reminders  = nm.due_reminders().len();
    let trash_count    = nm.trash.len();

    let most_versioned = nm.notes.iter().max_by_key(|n| n.versions.len());
    let most_words     = nm.notes.iter().max_by_key(|n| n.total_words());

    let all_tags: Vec<String> = nm.notes.iter().flat_map(|n| n.tags.iter().cloned()).collect();
    let mut tag_counts: std::collections::HashMap<String, usize> = Default::default();
    for tag in &all_tags { *tag_counts.entry(tag.clone()).or_insert(0) += 1; }
    let mut sorted_tags: Vec<_> = tag_counts.iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(a.1));

    let c = "\x1b[1;36m";
    let y = "\x1b[1;33m";
    let g = "\x1b[1;32m";
    let m = "\x1b[1;35m";
    let d = "\x1b[2m";
    let r = "\x1b[0m";
    let w = "\x1b[1;37m";

    println!("{c}  ╔══ {y}{w}STATISTICS DASHBOARD{r}{c}══════════════════╗{r}");
    println!("{c}  ║{r}");
    println!("{c}  ║{r}  {w}Total Notes{r}         {g}{total_notes:>6}{r}");
    println!("{c}  ║{r}  {w}Total Versions{r}      {m}{total_versions:>6}{r}");
    println!("{c}  ║{r}  {w}Pinned Notes{r}        {y}{pinned:>6}{r}");
    println!("{c}  ║{r}  {w}Avg Versions/Note{r}   {g}{avg_versions:>6.1}{r}");
    println!("{c}  ║{r}  {w}Total Words{r}         {g}{total_words:>6}{r}");
    println!("{c}  ║{r}  {w}Latest Chars{r}        {m}{total_chars:>6}{r}");
    println!("{c}  ║{r}  {w}Due Reminders 🔔{r}    {y}{due_reminders:>6}{r}");
    println!("{c}  ║{r}  {w}Trash (undo){r}        {d}{trash_count:>6}{r}");
    println!("{c}  ║{r}");

    if let Some(n) = most_versioned {
        println!("{c}  ║{r}  {w}Most Versioned:{r}  #{} {} {d}({} versions){r}",
            n.id, n.title, n.versions.len());
    }
    if let Some(n) = most_words {
        println!("{c}  ║{r}  {w}Most Words:{r}      #{} {} {d}({} words){r}",
            n.id, n.title, n.total_words());
    }
    println!("{c}  ║{r}");

    if !sorted_tags.is_empty() {
        println!("{c}  ║{r}  {w}Top Tags:{r}");
        for (tag, count) in sorted_tags.iter().take(5) {
            println!("{c}  ║{r}    {d}#{tag}{r}  {count}");
        }
    }
    println!("{c}  ║{r}");

    println!("{c}  ║{r}  {w}Note Breakdown:{r}");
    let mut sorted_notes: Vec<&crate::note::Note> = nm.notes.iter().collect();
    sorted_notes.sort_by(|a, b| b.pinned.cmp(&a.pinned));

    for note in sorted_notes {
        let bar: String = "█".repeat(note.versions.len().min(20));
        let pin = if note.pinned { "★ " } else { "  " };
        let due = if note.is_reminder_due() { "🔔" } else { "  " };
        let col = note.color.to_ansi();
        println!("{c}  ║{r}  {y}{pin}{r}{due}{col}#{:<3} {d}{:<20}{r} {g}{bar}{r} {d}{}v {}w{r}",
            note.id,
            &note.title.chars().take(20).collect::<String>(),
            note.versions.len(),
            note.total_words());
    }

    println!("{c}  ║{r}");
    // Show due reminders detail
    let due = nm.due_reminders();
    if !due.is_empty() {
        println!("{c}  ║{r}  {y}⚠  Due Reminders:{r}");
        for n in &due {
            if let Some(r) = n.reminder {
                println!("{c}  ║{r}    #{} {} — {}", n.id, n.title, format_timestamp(r));
            }
        }
        println!("{c}  ║{r}");
    }

    println!("{c}  ╚══════════════════════════════════════════════╝{r}");
}