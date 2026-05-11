# 📒 NOTES — CLI Note-Taking App in Rust

A fully-featured terminal-based note-taking application built from scratch in Rust. Supports versioning, full-text search, tags, reminders, note linking, export, password protection, trash bin, and a live statistics dashboard — all from the command line, zero dependencies beyond Rust itself.

---

## 📸 Preview

```
  ███╗   ██╗ ██████╗ ████████╗███████╗███████╗
  ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
  ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
  ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
  ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
  ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝

  3 notes · 1 pinned · 8 versions · autosave ON

  ┌─ MAIN MENU ─────────────────────────────────────────┐
  │  1.  Create Note                         [new]
  │  2.  Find / Open Note                     [→]
  │  3.  Show All Notes
  ...
```

---

## 🚀 Getting Started — Run From Scratch

### Step 1 — Install Rust

If you don't have Rust installed:

**Windows:**
- Download and run [rustup-init.exe](https://rustup.rs/)
- Follow the on-screen instructions (choose default installation)
- Restart your terminal after installation

**Linux / macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```
You should see something like `rustc 1.78.0` and `cargo 1.78.0`.

---

### Step 2 — Download the Project

**Option A — Clone with Git:**
```bash
git clone https://github.com/Pratham-gupta610/Notes_CLI.git
cd notes-cli
```

**Option B — Download ZIP:**
- Click the green **Code** button on GitHub → **Download ZIP**
- Extract the ZIP
- Open a terminal and `cd` into the extracted folder

---

### Step 3 — Build and Run

```bash
cargo run
```

That's it. Cargo (Rust's build tool) will automatically compile the project and launch the app. First build may take 30–60 seconds — subsequent runs are instant.

**For a faster optimized build:**
```bash
cargo build --release
```
Then run the binary directly:
```bash
# Windows
.\target\release\notes.exe

# Linux / macOS
./target/release/notes
```

---

### Step 4 — First Launch

On first launch you'll see the logo and main menu. A `DATA_STORAGE.txt` file will be created automatically in the project folder when you save your first note. You don't need to create it manually.

---

## 📁 Project Structure

```
notes-cli/
├── src/
│   ├── main.rs                  # Entry point — loads storage, launches UI
│   ├── note/
│   │   ├── mod.rs
│   │   └── note.rs              # Note, Version, NoteColor structs + IST timestamps
│   ├── manager/
│   │   ├── mod.rs
│   │   └── note_manager.rs      # All business logic — create, edit, delete, search
│   ├── storage/
│   │   ├── mod.rs
│   │   └── storage.rs           # Save/load notes from DATA_STORAGE.txt
│   ├── ui/
│   │   └── mod.rs               # All terminal menus, prompts, colors
│   ├── search/
│   │   └── mod.rs               # Full-text, tag, title, content search
│   ├── stats/
│   │   └── mod.rs               # Statistics dashboard
│   └── export/
│       └── mod.rs               # Markdown / plain text export & import
├── DATA_STORAGE.txt             # Auto-created — your notes live here
├── Cargo.toml                   # Project manifest
└── README.md
```

---

## ✨ Features

### 📝 Note Management
- Create, view, edit, and delete notes
- Every edit creates a new **timestamped version** — nothing is ever lost
- Edit a specific version in place — choose to **append** or **fully replace**
- Delete individual versions with automatic renumbering
- **Trash Bin** — deleted notes are recoverable until you empty trash

### 🗂 Organization
- **Tags** — multiple tags per note, bulk add/remove across all notes
- **Colors** — red, green, blue, yellow, magenta, cyan labels per note
- **Pin notes** — shown first with an orange `PINNED` label
- **Note linking** — connect related notes by ID, view linked notes inline

### 🔍 Search
- Full-text search across title, tags, and all version content
- Filter by tag, title, or content keyword individually

### 📊 Version Control
- Every edit saves a new version with its own timestamp, word count, and char count
- **Line-by-line diff** between any two versions
- Current version labeled `[current]` and shown at the top of the list

### 🗑 Trash Bin
- Deleted notes go to trash, not permanently deleted
- Restore by note ID or restore all at once
- Permanently delete individual notes from trash or empty all
- Quick undo prompt appears immediately after deleting a note

### 📅 Reminders
- Set reminders by minutes, hours, or exact seconds from now
- Due reminders show a bold banner + audio alert on main menu
- Reminder auto-clears after it fires

### 🔒 Password Protection
- Lock individual notes with a password
- Wrong password blocks access entirely
- Set or remove password any time

### 📤 Export / Import
- Export any note to `.md` (Markdown) with full version history
- Export to `.txt` (plain text)
- Import any `.md` or `.txt` file as a new version of a note

### 📈 Statistics Dashboard
- Total notes, versions, pinned count, average versions per note
- Total word counts across all notes
- Top 5 tags by frequency
- Visual bar chart showing versions per note
- Due reminders summary

### 💾 Storage & Autosave
- All notes saved to `DATA_STORAGE.txt` automatically
- Autosave after every action (can be toggled off in Settings)
- Manual save and load from the main menu
- Legacy format support — older saves load cleanly

### 🎨 Templates
- Blank note
- Meeting (Attendees, Agenda, Action Items, Notes)
- To-Do list
- Journal
- Custom — type your own template text

---

## ⌨️ Controls

### Main Menu

| Option | Action |
|--------|--------|
| `1` | Create note |
| `2` | Find / open note by ID |
| `3` | Show all notes |
| `4` | Search notes |
| `5` | Pinned notes |
| `6` | Statistics dashboard |
| `7` | Delete all notes |
| `8` | Save notes |
| `9` | Load notes |
| `10` | Settings |
| `11` | Help |
| `12` | Exit |
| `13` | Trash Bin |

### Note Detail Menu

| Key | Action |
|-----|--------|
| `a` | Add new version (edit note) |
| `b` | Pin / Unpin |
| `c` | Set color |
| `d` | Tag editor |
| `e` | Diff two versions |
| `f` | Export (.md or .txt) |
| `g` | Import .md/.txt as new version |
| `h` | Password protect |
| `i` | Set reminder |
| `j` | Link to another note |
| `k` | Delete note (with instant undo prompt) |
| `m` | View linked notes |
| `n` | Delete specific version |
| `o` | Edit specific version |
| `0` | Back |

### Editing Notes
- Type your content across multiple lines
- Type `;;` on a new line and press Enter to finish and save

---

## 💡 Tips

- Notes auto-sort with pinned ones at the top
- The `[current]` tag always shows on the latest version
- Reminders only fire when you return to the main menu
- Linking is one-directional — A→B does not automatically create B→A
- Export creates the file in the same folder you ran the app from
- Trash is in-memory only — if you exit without saving, trash is lost
- On Windows, reminder sounds require system volume to be unmuted and Focus Assist (DND) to be off

---

## 🗺 Roadmap

- [ ] `serde_json` storage for robustness (handles newlines in content properly)
- [ ] Password hashing with `bcrypt`
- [ ] Unit tests for core logic
- [ ] Background thread for reminder alerts
- [ ] Note categories / folders
- [ ] Markdown preview in terminal
- [ ] Search result highlighting

---

## 🤝 Contributing

Pull requests are welcome. For major changes please open an issue first to discuss what you'd like to change.

---

## 👤 Author

Built by Pratham Gupta a 1st year CS student learning Rust by building real things.

---

## 📄 License

MIT — free to use, modify, and distribute.
