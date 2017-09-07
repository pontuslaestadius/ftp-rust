use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};
use std::time;
use std::io::{Error, ErrorKind};

pub mod server;

use std::thread;

pub fn byte_to_string(bytes: &[u8], range: usize) -> String {
    let mut string = String::new();
    for i in 0..range {
        string.push(bytes[i] as char);
    }
    string
}

pub fn start_server(port: u16) {
    thread::spawn(move|| {server::Server::host(port);});
}

fn private_client() {
    let address = "127.0.0.1:19005";
    let mut stream = match connect(address) {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("client: caught an error while connecting, {}", e);
            panic!("client: unrecoverable error, {}", e);
        },
    };
    println!("client: connected to {:?}", stream.peer_addr().unwrap());

    let path = "examples/receive/foo.txt";

    let delay = time::Duration::from_millis(120);
    thread::sleep(delay);

    println!("client: i want '{}'", path);
    match receive(stream, path) {
        Ok(()) => (),
        Err(e) => eprintln!("unable to write to server: {}", e),
    }
}

pub fn start_client() {
    thread::spawn(|| {private_client();});
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

// Writes all the content from the buffer to the stream.
pub fn send(stream: &mut TcpStream, buf: &mut Buffer) -> Result<(), io::Error> {
    stream.write_all(buf.get())?;
    Ok(())
}

pub fn receive(mut stream: TcpStream, path: &str) -> Result<(), io::Error>{
    let _ = stream.write_all(path.as_bytes());
    println!("client: waiting for response from {}...",
             stream.peer_addr().unwrap());


    let (string, c) = read_socket(&mut stream, 5).unwrap();
    println!("client: received {}b read as: '{}'", c, string);

    //let mut f = File::create(path); // TODO use a different name path.
    Ok(())
}

pub fn read_socket<'a>
(stream: &mut TcpStream, timeout_sec: usize) -> Result<(String, usize), io::Error> {
    let mut buffer = [0; 500]; // TODO improve length
    let mut tries = 0;
    let mut c;
    let increment_delay = 500;
    let timeout = (timeout_sec as f64/(increment_delay as f64*0.001)) as usize;
    loop {
        c = stream.read(&mut buffer[..]).unwrap();
        if c == 0 {
            let delay = time::Duration::from_millis(increment_delay);
            thread::sleep(delay);
        } else {
            break;
        }

        tries += 1;
        if tries > timeout {
            return Err(Error::new(ErrorKind::Other, "timeout"));
        }
    };
    Ok((byte_to_string(&buffer, c), c))
}

pub fn progress_bar(buf: Buffer) {
    // TODO (Optional)
    // Displays the progress of the download in the terminal.
    // Example:
    // overview:
    // size: 30KB,
    // elapse: 2s,
    // speed: 15KB/s,
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

    pub fn get(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}