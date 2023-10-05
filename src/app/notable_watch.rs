use super::note_window;

use super::{app_commands::{self, send_command}, app_state::NoteState};

use druid::{Selector, ExtEventSink, text::EditableText};

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

use std::{time::Duration, sync::mpsc::Sender, fs};
use log::error;


pub const NOTABLE_DIR: &str = "/home/khashayar/.notable/notes/";
pub struct NotableWatch{
    launcher_handle: ExtEventSink
}
impl NotableWatch {
    pub fn new(launcher_handle: ExtEventSink) -> Self {
        NotableWatch {
            launcher_handle: launcher_handle
        }
    }
    pub fn watch(&mut self) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;

        debouncer
            .watcher()
            .watch(NOTABLE_DIR.as_ref(), RecursiveMode::Recursive)?;

        debouncer
            .cache()
            .add_root(NOTABLE_DIR, RecursiveMode::Recursive);

        for result in rx {
            match result {
                Ok(events) => {
                    send_command(self.launcher_handle.clone(), app_commands::UPDATE_NOTES);
                }
                Err(errors) => error!("an error occured while watching notable notes directory!"),
            }
        }

        Ok(())
    }

}