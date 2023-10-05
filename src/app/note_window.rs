// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use super::app_state::{AppState, NoteState};
use super::notes_meta::NoteMeta;
use pulldown_cmark::{Event as ParseEvent, Options, Parser, Tag, HeadingLevel};
use mouse_position::mouse_position::{Mouse};
use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::widget::{prelude::*, Flex};
use druid::widget::{Controller, LineBreaking, RawLabel, Scroll, Split, TextBox};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, FontFamily, FontStyle, FontWeight,
    Handled, Lens, LocalizedString, Menu, Selector, Target, Widget, WidgetExt, WindowDesc,
    WindowId,WindowLevel, WindowHandle, LensExt, Point, Vec2
};
use html_parser::Dom;
use html_parser::Element;
use html_parser::Error;

use super::app_commands;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Minimal Markdown");

const SPACER_SIZE: f64 = 8.0;
const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);
const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");


/// A controller that rebuilds the preview when edits occur
struct RichTextRebuilder;

struct Delegate;

impl<T: Data> AppDelegate<T> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut T,
        _env: &Env,
    ) -> Handled {
        if let Some(url) = cmd.get(OPEN_LINK) {
            #[cfg(not(target_arch = "wasm32"))]
            open::that_in_background(url);
            #[cfg(target_arch = "wasm32")]
            tracing::warn!("opening link({}) not supported on web yet.", url);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
pub fn create_note_window(parent_handle: &WindowHandle, note_index: usize, title: &str, pos: Point, size: Size) -> WindowDesc<AppState> {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget(note_index, title))
        .title(title)
        .show_titlebar(false)
        .resizable(true)
        .set_level(WindowLevel::Tooltip(parent_handle.clone()))
        .set_always_on_top(false)
        .set_position(pos)
        .window_size(size);
        main_window
}

struct DragWindowController {
    pub init_mouse_pos: Option<Point>,
    pub init_window_size: Option<Size>,
    pub init_window_pos: Option<Point>,
    pub resizing: bool,
    pub moving: bool,
    pub title: String,
    pub note_index: usize
}

impl DragWindowController {
    pub fn new(note_index: usize, title: String) -> Self {
        DragWindowController { 
            init_mouse_pos: None,
            init_window_pos: None,
            init_window_size: None,
            resizing: false,
            moving: false,
            title: title,
            note_index: note_index
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for DragWindowController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(me) if me.buttons.has_left() => {
                ctx.window().bring_to_front_and_focus();
                let position = Mouse::get_mouse_position();
                match position {
                        Mouse::Position { x, y } => self.init_mouse_pos = Some(Point { x: x.into(), y: y.into() }),
                        Mouse::Error => println!("Error getting mouse position"),
                }
                if(me.mods.alt()) {
                    self.resizing = true;
                    self.init_window_size = Some(ctx.window().get_size());
                } else {
                    self.moving = true;
                    self.init_window_pos = Some(ctx.window().get_position());
                }
                ctx.set_active(true);
            }
            Event::MouseMove(me) if ctx.is_active() && me.buttons.has_left() => {
                if let Some(init_mouse_pos) = self.init_mouse_pos {
                    let position = Mouse::get_mouse_position();
                    match position {
                            Mouse::Position { x, y } => {
                                if(self.resizing) {
                                    if(!me.mods.alt()) {
                                        self.resizing = false;
                                    } else {
                                        if let Some(init_window_size) = self.init_window_size {
                                            let new_size = Size { 
                                                width: init_window_size.width + (x as f64 - init_mouse_pos.x), 
                                                height: init_window_size.height + (y as f64 - init_mouse_pos.y) 
                                            };
                                            ctx.window().set_size(new_size);
                                        }
                                    }
                                } else if(self.moving) {
                                    if let Some(init_window_pos) = self.init_window_pos {
                                        let new_pos = Point { x: init_window_pos.x + (x as f64 - init_mouse_pos.x), y: init_window_pos.y + (y as f64 - init_mouse_pos.y) };
                                        ctx.window().set_position(new_pos);
                                    }
                                }
                            },
                            Mouse::Error => println!("Error getting mouse position"),
                    }
                }
            }
            Event::MouseUp(me) if ctx.is_active() => {
                self.init_mouse_pos = None;
                self.moving = false;
                self.resizing = false;
                let pos = ctx.window().get_position();
                let size = ctx.window().get_size();
                ctx.submit_command((app_commands::NOTE_WINDOW_MODIFIED.with(NoteMeta { 
                    title: self.title.clone(),
                    pos: vec![pos.x as usize, pos.y as usize],
                    size: vec![size.width as usize, size.height as usize],
                    z_index: 0
                })));
                ctx.set_active(false)
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}

fn build_root_widget(note_index: usize, title: &str) -> impl Widget<AppState> {
    let label = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::from_hex_str("#1f1c0c").expect("Bad color code!"))
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(AppState::notes.index(note_index).then(NoteState::rendered))
            .expand_width()
            .padding((SPACER_SIZE * 4.0, SPACER_SIZE))
    )
    .vertical()
    .border(Color::from_hex_str("#47321a").expect("Bad color code!"), 1.0)
    .background(Color::from_hex_str("#f7e26a").expect("Bad color code!"))
    .controller(DragWindowController::new(note_index, title.to_string()))
    .expand();
    label
}

fn parse_html(text: &str) -> Dom {
    match Dom::parse(text) {
        Ok(dom) => {
            return dom;
        },
        Error => panic!("Failed to parse html!")
    };
}
fn get_dir_attr(element: &Element) -> String {
    let dir = match element.attributes.get("dir") {
        Some(attr) => {
            return match attr {
                Some(s) => return s.to_string(),
                None => return String::from("None")
            }
        },
        None => return String::from("None")
    };
}
/// Parse a markdown string and generate a `RichText` object with
/// the appropriate attributes.
pub fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut tag_stack = Vec::new();

    let parser = Parser::new_ext(text, Options::ENABLE_STRIKETHROUGH);
    for event in parser {
        match event {
            ParseEvent::Start(tag) => {
                tag_stack.push((current_pos, tag));
            }
            ParseEvent::Text(txt) => {
                builder.push(&txt);
                current_pos += txt.len();
            }
            ParseEvent::End(end_tag) => {
                let (start_off, tag) = tag_stack
                    .pop()
                    .expect("parser does not return unbalanced tags");
                assert_eq!(end_tag, tag, "mismatched tags?");
                add_attribute_for_tag(
                    &tag,
                    builder.add_attributes_for_range(start_off..current_pos),
                );
                if add_newline_after_tag(&tag) {
                    builder.push("\n\n");
                    current_pos += 2;
                }
            }
            ParseEvent::Code(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
            }
            ParseEvent::Html(txt) => {
                let dom = parse_html(&txt);
                let first_node = dom.children.get(0)
                    .expect("Bad Html Node!");
                let first_element = first_node.element()
                    .expect("Bad Html Element!");
                
                let dir = get_dir_attr(first_element);
                println!("{}", dom.to_json_pretty().expect("s"));
                if(first_element.name == "div" && dir == "rtl"){
                    for child in &first_element.children {
                        let mut t = String::from("\u{202b}");
                        t.push_str(match child.text() {
                            Some(s) => s,
                            None => ""
                        });
                        builder
                            .push(&t);
                    }
                } else {
                builder
                    .push(&txt)
                    .font_family(FontFamily::MONOSPACE)
                    .text_color(BLOCKQUOTE_COLOR);
                }
                current_pos += txt.len();
            }
            ParseEvent::HardBreak => {
                builder.push("\n\n");
                current_pos += 2;
            }
            _ => (),
        }
    }
    builder.build()
}

fn add_newline_after_tag(tag: &Tag) -> bool {
    !matches!(
        tag,
        Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
    )
}

fn add_attribute_for_tag(tag: &Tag, mut attrs: AttributesAdder) {
    match tag {
        Tag::Heading(lvl,_l,_k) => {
            let font_size = match lvl {
                HeadingLevel::H1 => 38.,
                HeadingLevel::H2 => 32.0,
                HeadingLevel::H3 => 26.0,
                HeadingLevel::H4 => 20.0,
                HeadingLevel::H5 => 16.0,
                _ => 12.0,
            };
            attrs.size(font_size).weight(FontWeight::BOLD);
        }
        Tag::BlockQuote => {
            attrs.style(FontStyle::Italic).text_color(BLOCKQUOTE_COLOR);
        }
        Tag::CodeBlock(_) => {
            attrs.font_family(FontFamily::MONOSPACE);
        }
        Tag::Emphasis => {
            attrs.style(FontStyle::Italic);
        }
        Tag::Strong => {
            attrs.weight(FontWeight::BOLD);
        }
        Tag::Strikethrough => {
            attrs.strikethrough(true);
        }
        Tag::Link(_link_ty, target, _title) => {
            attrs
                .underline(true)
                .text_color(LINK_COLOR)
                .link(OPEN_LINK.with(target.to_string()));
        }
        // ignore other tags for now
        _ => (),
    }
}

#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window_id: Option<WindowId>, _app_state: &AppState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(
        target_os = "windows",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd"
    ))]
      {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    base.entry(
        Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut().enabled(false))
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste()),
    )
}