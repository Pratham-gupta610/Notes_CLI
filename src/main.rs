mod note;
mod manager;
mod storage;
mod ui;
mod search;
mod stats;
mod export;

use manager::NoteManager;

fn main() {
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/DATA_STORAGE.md");

    ui::clear_screen();
    ui::print_logo();

    let mut note_manager =
        storage::load_notes_from_file(file_path).unwrap_or_else(|_| NoteManager::new());

    ui::main_menu(&mut note_manager, file_path);
}