
use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::net::TcpListener;
use std::thread;

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

pub struct Server {
    connections: Vec<TcpListener>,
    pub assigner: TcpListener,
    pub running: Running,
}

// Holds the state of the server.
pub enum Running {
    Yes(thread::JoinHandle<()>),
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
    pub fn hopp(&self) {
    }

    pub fn new(address: &str, port: &str) -> Result<Server, io::Error>  {
        let listener = TcpListener::bind([address,":",port].concat())?;

        Ok(Server {
            connections: Vec::new(),
            assigner: listener,
            running: Running::No
        })
    }

    pub fn add(&mut self, listener: TcpListener) {
        self.connections.push(listener);
    }

    pub fn running(&self) -> bool {
        self.running != Running::No
    }

    pub fn start(&mut self) {
        if !self.running() {
            //TODO introduce threading.
            self.read_client();
        }
    }

    pub fn read_client(&mut self) {
        let (mut stream, addr) = self.assigner.accept().unwrap();
        println!("Accepted connection: {:?}", addr);

        let mut string: String = String::new();
        stream.read_to_string(&mut string);

        println!("handle_client: {}", string);
    }

    pub fn stop(&self) {
        if self.running() {
            // TODO
        }
    }
}