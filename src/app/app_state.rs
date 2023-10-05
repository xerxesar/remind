use druid::Data;
use druid::text::RichText;
use druid::Lens;

pub const MAX_NOTES_COUNT: usize = 8;

#[derive(Debug, Clone, Data, Lens)]
pub struct NoteState {
    pub title: String,
    pub rendered: RichText
}
#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    pub name: String,
    pub notes: [NoteState;MAX_NOTES_COUNT],
    pub notes_count: usize,
    pub closeable: bool,
}