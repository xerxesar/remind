use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    match TcpStream::connect("127.0.0.1:9982") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"STICKABLE_HOTKEY_EVENT\0";

            stream.write(msg).unwrap();

            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut name = String::new();
            let mut bodyRaw: Vec<u8> = Vec::new(); //New Vector with size of Content   
            reader.read_until(0,&mut bodyRaw); //Get the Body Content.
            let body = String::from_utf8(bodyRaw).expect("Failed to read tcp stream!");

            println!("{}",body);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}