
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
    pub fn host(port: &str) {
        let mut server = Server::new("127.0.0.1", port).unwrap();
        println!("server: ready");

        for stream in server.stream.incoming() {
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                Server::handle_client(&mut stream);
            });
        }
        println!("server: shutting down");
    }

    fn handle_client(stream: &mut TcpStream) {
        println!("server: connected {:?}", stream.peer_addr().unwrap());
        let mut buf = Vec::new();
        let mut tries = 0;
        loop {
            let c = stream.read(&mut buf).unwrap();
            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);
            println!("server: read {} bytes", buf.len());

            tries += 1;
            if tries > 5 {
                println!("server: nothing received, closing connection.");
                return;
            }
        };

        println!("server: received '{}' bytes", buf.len());
    }

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

    pub fn new(address: &str, port: &str) -> Result<Server, io::Error>  {
        let listener = TcpListener::bind([address,":",port].concat())?;

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