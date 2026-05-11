// note/note.rs

use std::time::{SystemTime, UNIX_EPOCH};

// IST = UTC + 5 hours 30 minutes = 19800 seconds
const TZ_OFFSET_SECS: u64 = 5 * 3600 + 30 * 60;

pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn format_timestamp(ts: u64) -> String {
    if ts == 0 {
        return "unknown".to_string();
    }

    let local_ts = ts + TZ_OFFSET_SECS;

    let secs_in_day = local_ts % 86400;
    let days_total  = local_ts / 86400;

    let hour   = secs_in_day / 3600;
    let minute = (secs_in_day % 3600) / 60;
    let second = secs_in_day % 60;

    // Proper year calculation with leap years
    let mut year = 1970u64;
    let mut remaining_days = days_total;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining_days < days_in_year { break; }
        remaining_days -= days_in_year;
        year += 1;
    }

    let month_lengths = month_days(year);
    let mut month = 0usize;
    for (i, &ml) in month_lengths.iter().enumerate() {
        if remaining_days < ml { month = i; break; }
        remaining_days -= ml;
    }

    let day = remaining_days + 1;
    let month_names = ["Jan","Feb","Mar","Apr","May","Jun",
                       "Jul","Aug","Sep","Oct","Nov","Dec"];

    format!("{:02} {} {}  {:02}:{:02}:{:02} IST",
        day, month_names[month], year, hour, minute, second)
}

fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn month_days(year: u64) -> [u64; 12] {
    [31, if is_leap(year) { 29 } else { 28 }, 31, 30, 31, 30,
     31, 31, 30, 31, 30, 31]
}

// ── Color label ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum NoteColor {
    Default, Red, Green, Blue, Yellow, Magenta, Cyan,
}

impl NoteColor {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            NoteColor::Default => "\x1b[0m",
            NoteColor::Red     => "\x1b[1;31m",
            NoteColor::Green   => "\x1b[1;32m",
            NoteColor::Blue    => "\x1b[1;34m",
            NoteColor::Yellow  => "\x1b[1;33m",
            NoteColor::Magenta => "\x1b[1;35m",
            NoteColor::Cyan    => "\x1b[1;36m",
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteColor::Default => "default", NoteColor::Red  => "red",
            NoteColor::Green   => "green",   NoteColor::Blue => "blue",
            NoteColor::Yellow  => "yellow",  NoteColor::Magenta => "magenta",
            NoteColor::Cyan    => "cyan",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "red" => NoteColor::Red, "green" => NoteColor::Green,
            "blue" => NoteColor::Blue, "yellow" => NoteColor::Yellow,
            "magenta" => NoteColor::Magenta, "cyan" => NoteColor::Cyan,
            _ => NoteColor::Default,
        }
    }
    pub fn menu_str() -> &'static str {
        "  1) red  2) green  3) blue  4) yellow  5) magenta  6) cyan  0) default"
    }
    pub fn from_menu(n: &str) -> Self {
        match n.trim() {
            "1" => NoteColor::Red,    "2" => NoteColor::Green,
            "3" => NoteColor::Blue,   "4" => NoteColor::Yellow,
            "5" => NoteColor::Magenta,"6" => NoteColor::Cyan,
            _   => NoteColor::Default,
        }
    }
}

// ── Version ───────────────────────────────────────────────────────────────────

pub struct Version {
    pub version_number: u32,
    pub content: String,
    pub created_at: u64,         // unix timestamp, per-version
}

impl Version {
    pub fn new(version_number: u32, content: String) -> Self {
        Version { version_number, content, created_at: now_timestamp() }
    }
    pub fn new_with_ts(version_number: u32, content: String, created_at: u64) -> Self {
        Version { version_number, content, created_at }
    }
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
    pub fn char_count(&self) -> usize {
        self.content.chars().count()
    }
    /// Formatted display line including per-version timestamp
    // pub fn display_line(&self) -> String {
    //     format!(
    //         "\x1b[1;36m  v{}\x1b[0m \x1b[2m│ {} words │ {} chars │ {}\x1b[0m\n      \x1b[0;37m{}\x1b[0m",
    //         self.version_number,
    //         self.word_count(),
    //         self.char_count(),
    //         format_timestamp(self.created_at),
    //         self.content
    //     )
    // }
    pub fn display_line(&self, is_current: bool) -> String {
    let tag = if is_current { " \x1b[1;32m[current]\x1b[0m" } else { "" };
    format!(
        "\x1b[1;36m  v{}\x1b[0m \x1b[2m│ {} words │ {} chars │ {}{}\x1b[0m\n      \x1b[0;37m{}\x1b[0m",
        self.version_number,
        self.word_count(),
        self.char_count(),
        format_timestamp(self.created_at),
        tag,
        self.content
    )
}

}

// ── Note ──────────────────────────────────────────────────────────────────────

pub struct Note {
    pub id: u32,
    pub title: String,
    pub versions: Vec<Version>,
    pub pinned: bool,
    pub tags: Vec<String>,
    pub color: NoteColor,
    pub created_at: u64,
    pub updated_at: u64,
    pub password: Option<String>,
    pub reminder: Option<u64>,
    pub links: Vec<u32>,
}

impl Note {
    pub fn new(id: u32, title: String) -> Self {
        let ts = now_timestamp();
        Note {
            id, title,
            versions: Vec::new(),
            pinned: false,
            tags: vec!["general".to_string()],
            color: NoteColor::Default,
            created_at: ts, updated_at: ts,
            password: None, reminder: None,
            links: Vec::new(),
        }
    }

    pub fn add_version(&mut self, new_content: String) {
        let vn = self.versions.len() as u32 + 1;
        self.versions.push(Version::new(vn, new_content));
        self.updated_at = now_timestamp();
    }
pub fn edit_version(&mut self, version_number: u32, new_content: String) -> bool {
    if let Some(vin) = self.versions.iter_mut().find(|temp| temp.version_number == version_number) {
        vin.content = new_content;
        vin.created_at = now_timestamp();
        self.updated_at = now_timestamp();
        true
    } else {
        false
    }
}
    pub fn get_latest_version(&self) -> Option<&Version> { self.versions.last() }

pub fn show_versions(&self) {
    let total = self.versions.len();
    for (i, v) in self.versions.iter().rev().enumerate() {
        println!("{}", v.display_line(i == 0));
    }
}  // i is 0 for latest version(v3), which gets the [current] tag after .rev()
// (i=0, v=v3)
// (i=1, v=v2)
// (i=2, v=v1)
    pub fn total_words(&self) -> usize {
        self.versions.iter().map(|v| v.word_count()).sum()
    }
    pub fn latest_word_count(&self) -> usize {
        self.get_latest_version().map(|v| v.word_count()).unwrap_or(0)
    }
    pub fn latest_char_count(&self) -> usize {
        self.get_latest_version().map(|v| v.char_count()).unwrap_or(0)
    }

    pub fn diff(&self, v1: u32, v2: u32) -> Option<String> {
        let ver1 = self.versions.iter().find(|v| v.version_number == v1)?;
        let ver2 = self.versions.iter().find(|v| v.version_number == v2)?;
        let lines1: Vec<&str> = ver1.content.lines().collect();
        let lines2: Vec<&str> = ver2.content.lines().collect();
        let mut out = format!("\x1b[1;33m  Diff v{} → v{}\x1b[0m\n", v1, v2);
        for i in 0..lines1.len().max(lines2.len()) {
            match (lines1.get(i), lines2.get(i)) {
                (Some(a), Some(b)) if a == b => out.push_str(&format!("  \x1b[2m  {}\x1b[0m\n", a)),
                (Some(a), Some(b)) => {
                    out.push_str(&format!("  \x1b[1;31m- {}\x1b[0m\n", a));
                    out.push_str(&format!("  \x1b[1;32m+ {}\x1b[0m\n", b));
                }
                (Some(a), None) => out.push_str(&format!("  \x1b[1;31m- {}\x1b[0m\n", a)),
                (None, Some(b)) => out.push_str(&format!("  \x1b[1;32m+ {}\x1b[0m\n", b)),
                _ => {}
            }
        }
        Some(out)
    }

    pub fn is_reminder_due(&self) -> bool {
        self.reminder.map(|r| r <= now_timestamp()).unwrap_or(false)
    }
}