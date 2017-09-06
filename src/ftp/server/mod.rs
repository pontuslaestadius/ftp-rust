
use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn send(path: &str) -> Result<(), io::Error> {
    // TODO
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(())
}

// Wrapper for a tcplistener
pub struct Server {
    pub stream: TcpListener,
    pub running: Running,
}

// Holds the state of the server.
pub enum Running {
    Yes,
    No,
}

impl Running {

    pub fn state(&self) -> bool {
        match self {
            &Running::No => false,
            _ => true
        }
    }
}

impl PartialEq for Running {
    fn eq(&self, other: &Running) -> bool {
        self.state() == other.state()
    }

    fn ne(&self, other: &Running) -> bool {
        !self.eq(other)
    }
}

impl Server {
    pub fn host(port: u16) {
        let mut server = Server::new("127.0.0.1", port).unwrap();
        println!("server: ready");

        for stream in server.stream.incoming() {
            let mut stream = stream.unwrap();
            thread::spawn(move || {
                Server::handle_client(stream);
            });
        }
        println!("server: shutting down");
    }

    fn handle_client(mut stream: TcpStream) {
        println!("server: connected {:?}", stream.peer_addr().unwrap());
        //let mut buf = Vec::new();
        let mut buffer = [0; 50];

        let mut tries = 0;
        let mut c;
        loop {
            c = stream.read(&mut buffer[..]).unwrap();
            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);

            if c != 0 {
                println!("server: read {} bytes", c);
                break;
            }

            tries += 1;
            if tries > 5 {
                println!("server: no data timeout ({})",
                         stream.peer_addr().unwrap());
                return;
            }
        };

        println!("server: received '{}' bytes", c);

        let mut string = String::new();
        for i in 0..c {
            string.push(buffer[i] as char);
        }

        println!("server: received {}B read as: '{}'", c, string);


        //let result = Server::action();
    }

    /*
    fn action(input: &str) {
        match input {
            _
        }
    }
    */

    fn send_file(stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
        let mut f = OpenOptions::new()
            .read(true)
            .truncate(false)
            .open(path)?;

        let mut content: String = String::new();
        f.read_to_string(&mut content);
        stream.write_all(content.as_bytes());
        Ok(())
    }

    pub fn new(address: &str, port: u16) -> Result<Server, io::Error>  {
        let port: String = port.to_string();
        let listener = TcpListener::bind([address,":",port.as_str()].concat())?;

        Ok(Server {
            stream: listener,
            running: Running::Yes
        })
    }

    pub fn running(&self) -> bool {
        self.running != Running::No
    }

    pub fn start(&mut self) {
        if !self.running() {
            // TODO this does nothing.
        }
    }

    /*
    pub fn read_client(&mut self) {
        let (mut stream, addr) = self.assigner.accept().unwrap();
        println!("server: Waiting for a message...");

        //let mut string: String = String::new();
        //let _ = stream.read_to_string(&mut string);


        //println!("server: received command '{}'", string);
    }
    */

    pub fn stop(&self) {
        if self.running() {
            // TODO
        }
    }
}