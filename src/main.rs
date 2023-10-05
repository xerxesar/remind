// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
mod app;
use app::App;

// Import other modules
use druid::{
    theme, Affine, AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens,
     LifeCycle, LifeCycleCtx,  PaintCtx, RenderContext, Size,
     UpdateCtx, Widget, WidgetExt, WindowDesc, WindowId, AppDelegate, DelegateCtx, Selector, Target, Rect
};
use druid::text::{RichText, EditableText};
use druid::widget::{Align, Checkbox, Controller, ControllerHost, Flex, TextBox};
use druid_shell::{Screen, WindowHandle};
use druid_shell::piet::Text;
use druid::lens::Unit;

use std::process::Command;
use std::sync::mpsc::{self, Sender};
use std::io::{BufReader,BufRead};
use std::net::{TcpListener, TcpStream};
use std::{thread, fs};
use std::io::Write;

use piet_common::{TextLayout, TextLayoutBuilder};


pub fn main()  -> notify::Result<()> {
    
    let mut app = App::new();
    
    app.launch();
    Ok(())
}





