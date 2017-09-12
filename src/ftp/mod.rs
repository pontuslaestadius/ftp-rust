use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};
use std::time;
use std::io::{Error, ErrorKind};

pub mod server;
pub mod client;
pub mod decode;
pub mod encode;
pub mod metadata;

use std::thread;

const STDBUF: usize = 1024;

// Converts a list of bytes until a specific range in to a string.
pub fn byte_to_string(bytes: &[u8], range: usize) -> String {
    let mut string = String::new();
    for i in 0..range {
        string.push(bytes[i] as char);
    }
    string
}

// Returns the string encased in a tag.
fn format_tag<'a, 'b> (tag: &'a str, cont: &'a str) -> String {
    [tag, "{", cont, "}"].concat()
}

// TODO move in to a seperate file?
fn get_file(path: &str) -> Result<Vec<String>, io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let path_string = path.to_string();

    let mut str: &str = "undefined.txt";

    // Splits the path by directory dividers so the
    // last split in the path is the file name.
    let split = path_string.split('/');
    for s in split {
        str = s;
    }

    let encoded = encode::file(f, str)?;
    println!("sending {} packet(s)", encoded.len());
    Ok(encoded)
}

pub fn start_server(address: &str, port: &str) {
    thread::spawn(move|| {server::host("127.0.0.1", "19005");});
}

pub fn read_console() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let len = input.len()-1;
    input.truncate(len);
    input
}

// Connects through TCP and return the stream.
pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

// Returns a Buffer with the content of the file.
pub fn get_buffer(path: &str) -> Result<Buffer, io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(Buffer::new(buf))
}

pub fn get(mut stream: &mut TcpStream, path: &str) -> Result<(), io::Error>{
    let _ = stream.write_all(["get ", path].concat().as_bytes());
    //println!("client: waiting for response from {}...", stream.peer_addr()?);

    let string = read_socket(&mut stream, 10)?;
    println!("received {}b", string.len());

    decode::generic(string)?;
    Ok(())
}

pub fn read_socket<'a>
(stream: &mut TcpStream, timeout_milli: usize) -> Result<String, io::Error> {
    let mut buffer = [0; STDBUF]; // TODO improve length
    let mut tries = 0;
    let mut c;
    let increment_delay = 25;
    let timeout = (timeout_milli/increment_delay as usize) as usize;

    let delay = time::Duration::from_millis(increment_delay);
    stream.set_read_timeout(None)?;
    loop {
        c = stream.read(&mut buffer[..])?;
        if c != 0 {
            break;
        }

        thread::sleep(delay);
        tries += 1;
        if tries > timeout {
            return Err(Error::new(ErrorKind::Other, "timeout"));
        }
    };
    Ok(byte_to_string(&buffer, c))
}

// TODO improve. Not implemented.
pub fn send_ask(stream: &mut TcpStream) -> Result<(), io::Error> {
    let mut files = "examples/files/foo.txt";
    let packets = encode::generic("disp", &mut files.to_string(), "")?;
    send_vec(stream, packets);
    Ok(())
}

pub fn send_get(stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    let encoded = get_file(path)?;
    send_vec(stream, encoded);
    Ok(())
}


pub fn send_vec(stream: &mut TcpStream, vec: Vec<String>) {
    for v in vec {
        let _ = stream.write_all(v.as_bytes());
    }
}

pub struct Buffer {
    buf: Vec<u8>,
}

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer {
            buf
        }
    }

    pub fn from_str(buf: &str) -> Buffer {
        Buffer {
            buf: buf.as_bytes().to_vec()
        }
    }

    pub fn get(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}