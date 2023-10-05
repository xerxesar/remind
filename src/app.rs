mod app_state;
mod app_commands;
mod notes_meta;
mod note_window;
mod notable_watch;
mod hotkey_server;
mod utils;

use hotkey_server::HotkeyServer;
use std::{fs, process::Command, thread, net::TcpListener, sync::mpsc};

use app_state::{AppState, NoteState, MAX_NOTES_COUNT};
use druid::{AppLauncher, ExtEventSink, text::{RichText, EditableText}, WindowDesc, Selector, Widget, widget::{TextBox, Flex, Align}, WindowId, DelegateCtx, Point, Size, AppDelegate, Target, Env};

use druid_shell::WindowHandle;
use global_hotkey::hotkey;
use log::info;
use notes_meta::{NotesMeta,NoteMeta};

use self::notable_watch::NotableWatch;

// define app constants
const WINDOW_TITLE: &str = "stickable_main_window_055aaf1a-6dc4-40cc-b761-be002a52fe97";



pub struct App {
    app_state: AppState,
    pub launcher: AppLauncher<AppState>,
    pub launcher_handle: ExtEventSink
}
impl App {
    pub fn new() -> Self {
    
    
        // initialize app_state
        let mut notes = Vec::<NoteState>::new();
        for i in 0..MAX_NOTES_COUNT {
            notes.push(NoteState { title: "".into(), rendered: RichText::new("".into())});
        }
        let app_state = AppState {
            name: "World".into(),
            notes: notes.clone().try_into().unwrap(),
            notes_count: 0,
            closeable: true,
        };

        // describe the main window
        let main_window = WindowDesc::new(App::build_root_widget())
            .title(WINDOW_TITLE)
            .set_position((0.0, 0.0))
            .window_size((0.0, 0.0));

        let notes_meta = notes_meta::NotesMeta::new()
            .load_notes_meta().to_owned();
        // initialize the launcher
        let launcher = AppLauncher::with_window(main_window)
        .delegate(Delegate {
            windows: Vec::new(),
            main_window: WindowHandle::default(),
            note_handles: Vec::new(),
            showing_notes: false,
            notes_meta: notes_meta
        })
        .log_to_console();
        let external_handle = launcher.get_external_handle();
        
        // initialize the hotkey server
        let mut hotkey_server = HotkeyServer::new(external_handle.clone());
        thread::spawn(move|| -> notify::Result<()> {
            hotkey_server.listen();
            Ok(())
        });

        // initialize notable watch
        let mut notable_watch = NotableWatch::new(external_handle.clone());
        thread::spawn(move|| {
            notable_watch.watch();
        });
        
        App {
            app_state: app_state,
            launcher: launcher,
            launcher_handle: external_handle.clone()
        }
    }
    pub fn launch(self) {
        self.launcher.launch(self.app_state)
        .expect("Failed to launch application")
    }




fn build_root_widget() -> impl Widget<AppState> {
    let layout = Flex::column();
    Align::centered(layout)
}

}


struct Delegate {
    windows: Vec<WindowId>,
    main_window: WindowHandle,
    note_handles: Vec<WindowHandle>,
    showing_notes: bool,
    notes_meta: NotesMeta
}
impl Delegate {
    fn show_notes(&mut self, ctx: &mut DelegateCtx, data: &mut AppState) {
        let mut new_note_index = 0;
        for note_index in 0..data.notes_count {
            let title = data.notes[note_index].title.as_str();
            let note_meta = self.notes_meta.data.notes.iter().find(|m| m.title == title);
            let (pos, size) = match note_meta {
                Some(m) => {
                    (
                        Point {x: m.pos[0] as f64, y: m.pos[1] as f64},
                        Size {width: m.size[0] as f64, height: m.size[1] as f64}
                    )
                },
                None => {
                    new_note_index+=1;
                    (
                        Point {x: (new_note_index as f64) * 50.0, y: (new_note_index as f64) * 50.0},
                        Size {width: 200.0, height: 300.0}
                    )
                },
            };
            let win_desc = note_window::create_note_window(&self.main_window, note_index, title, pos, size);
            
            ctx.new_window(win_desc);
        }   
    }    
    fn hide_notes(&mut self, ctx: &mut DelegateCtx, data: &mut AppState) {
        for i in 0..self.note_handles.len() {
            let note_handle = self.note_handles.get(i).expect("Failed to select note_handle!");
            note_handle.close();
        }
    }    
    fn update_notes(&mut self, ctx: &mut DelegateCtx, data: &mut AppState) {
                let entries = fs::read_dir(notable_watch::NOTABLE_DIR).unwrap();
                let mut note_index = 0;
                for entry in entries {
                    let path = entry.unwrap().path();
                    let path_str = path.to_str().expect("Bad file path value!");
                    let content = utils::open_md_file(path_str);
                    let parts: Vec<&str> = content.split("---").collect();
                    let metadata = parts[1].lines();
                    let title_line = metadata.filter(|line| line.starts_with("title:")).nth(0).expect("Note file does not have title field!");
                    let title = title_line.to_string();
                    let title = title.slice(7..title_line.len()).expect("Failed to retrieve note title!");
                    println!("{}", title);
                    let renderable = parts[2];
                    data.notes[note_index] = NoteState{ title: title.to_string(), rendered: note_window::rebuild_rendered_text(renderable) };
                    note_index += 1;
                }
                data.notes_count = note_index;
                println!("Updating notes {}", self.notes_meta.data.notes.len());
                
                self.notes_meta.data.notes.retain(
                |m| 
                    data.notes.iter().enumerate().position(
                        |(i, n)| {
                            println!("{} : {}", n.title, m.title);
                            n.title == m.title && i < data.notes_count
                        }
                    ).is_some()
                );
                println!("Updating notes {} ... ", self.notes_meta.data.notes.len());
                self.notes_meta.save_notes_meta();
    }    
}
impl AppDelegate<AppState> for Delegate {

    fn command(
            &mut self,
            ctx: &mut DelegateCtx,
            target: Target,
            cmd: &druid::Command,
            data: &mut AppState,
            env: &Env,
        ) -> druid::Handled {
            if(cmd.is(app_commands::TOGGLE_NOTES)) {
                if(self.showing_notes) {
                    println!("HIDE_NOTES command received :)");
                    self.hide_notes(ctx, data);    
                } else {
                    println!("SHOW_NOTES command received :)");
                    self.show_notes(ctx, data);
                }
                self.showing_notes = !self.showing_notes;
            } else if(cmd.is(app_commands::UPDATE_NOTES)) {
                println!("UPDATE_NOTES command received :)");
                self.update_notes(ctx, data);
            } else if(cmd.is(app_commands::NOTE_WINDOW_MODIFIED)) {
                println!("NOTE_WINDOW_MODIFIED command received :)");
                let new_note_meta = cmd.get_unchecked(app_commands::NOTE_WINDOW_MODIFIED);
                let mut i = 0;
                let mut old_meta: Option<NoteMeta> = None;
                let note_index = self.notes_meta.data.notes.iter().position(|n| n.title == new_note_meta.title);
                println!("{:#?}", new_note_meta);
                if(note_index.is_none()) {
                    self.notes_meta.data.notes.push(new_note_meta.clone());
                } else {
                    println!("{:#?}", self.notes_meta.data.notes[note_index.unwrap()]);
                    self.notes_meta.data.notes[note_index.unwrap()] = new_note_meta.clone();
                    println!("{:#?}", self.notes_meta.data.notes[note_index.unwrap()]);
                }
                self.notes_meta.save_notes_meta();
            } 
        druid::Handled::Yes
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        if(self.windows.len() == 0) {
            self.main_window = _handle;
            println!("hiding first window from taskbar");
            let mut command = Command::new("xprop");
            command.args([ "-name", WINDOW_TITLE,
                        "-f", "_NET_WM_STATE", "32a",
                        "-set", "_NET_WM_STATE", "_NET_WM_STATE_SKIP_TASKBAR"]);
            command.spawn()
                .expect("failed to execute process");
            self.update_notes(_ctx, _data);
        } else {
            self.note_handles.push(_handle);
        }
        info!("Window added, id: {:?}", id);
        self.windows.push(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        info!("Window removed, id: {:?}", id);
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}