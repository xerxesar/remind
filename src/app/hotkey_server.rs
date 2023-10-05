use std::{net::{TcpListener, TcpStream}, thread, io::{BufReader, BufRead, Write}};

use druid::{ExtEventSink, Target};

use crate::app::app_commands::{send_command, self};

use super::app_commands::TOGGLE_NOTES;

pub struct HotkeyServer {
    launcher_handle: ExtEventSink
}
impl HotkeyServer {
    pub fn new(launcher_handle: ExtEventSink) -> Self {
        HotkeyServer {
            launcher_handle: launcher_handle
        }
    }
    pub fn listen(mut self) {
        let listener = TcpListener::bind("127.0.0.1:9982").expect("Failed to start hotkey server on port 9982!");
    
            // accept connections and process them serially
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        let launcher_handle = self.launcher_handle.clone();
                        thread::spawn(move|| {
                            Self::handle_socket_stream(launcher_handle, stream);
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
    }
    fn handle_socket_stream(launcher_handle: ExtEventSink,mut stream: TcpStream) {

        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut name = String::new();
        let mut bodyRaw: Vec<u8> = Vec::new(); //New Vector with size of Content   
        reader.read_until(0,&mut bodyRaw); //Get the Body Content.
        let body = String::from_utf8(bodyRaw).expect("Failed to read tcp stream!");

        if(body == "STICKABLE_HOTKEY_EVENT\0"){
            println!("{}",body);
            send_command(launcher_handle.clone(), app_commands::TOGGLE_NOTES);
            let msg = b"STICKABLE_HOTKEY_EVENT:RECEIVED\0";
            stream.write(msg).unwrap();
        }
        
    }
}