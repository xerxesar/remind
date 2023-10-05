use druid::{Selector, ExtEventSink, Target};
use super::notes_meta::NoteMeta;

// druid command selectors
pub const TOGGLE_NOTES: Selector = Selector::new("stickable.toggle_notes");
pub const UPDATE_NOTES: Selector = Selector::new("stickable.update_notes");
pub const NOTE_WINDOW_MODIFIED: Selector<NoteMeta> = Selector::new("stickable.note_window_modified");

pub fn send_command(launcher_handler: ExtEventSink, command: Selector) {
    launcher_handler.submit_command(command, Box::new(()), Target::Global);
}