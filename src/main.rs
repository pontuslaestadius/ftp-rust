
pub mod server;


use server::*;
use std::net::TcpStream;
use std::io::prelude::*;
use std::{thread, time, env};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Handle environment variables.

    if args.len() > 1 {
        while args.len() > 1 {
            let query = args.swap_remove(1);
            match query.as_str() {
                "server" => start_server(),
                "client" => start_client(),
                _ => eprintln!("unknown command '{}'", query)
            };
            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);
        }
    } else {
        start_client();
    }

}

fn start_server() {
    thread::spawn(|| {private_server();});
}

fn private_server() {
    println!("server: starting");
    let mut server = server::Server::new("127.0.0.1", "9005").unwrap();
    println!("server: waiting...");
    server.start();
}

fn start_client() {
    println!("client: starting");
    let address = "127.0.0.1:9005";
    let mut stream = match ftp::connect(address) {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("caught an error while connecting, {}", e);
            panic!("caught an error while connecting");
        },
    };
    println!("client: connected to {:?}", stream.peer_addr().unwrap());

    let path = "examples/receive/foo.txt";
    //let mut buf = ftp::get_buffer(path).unwrap();
    ftp::receive(&mut stream, path);
}

pub mod ftp {

    use std::net::TcpStream;
    use std::io::prelude::*;
    use std::io;
    use std::fs::{File, OpenOptions};

    use std::thread;

    // Connects through TCP and return the stream.
    pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
        let mut stream = TcpStream::connect(address)?;
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

    // A non-blocking recieve.
    pub fn receive(stream: &mut TcpStream, path: &str) -> Result<(), io::Error>{
        stream.write_all(path.as_bytes());
        println!("client: I want a file from the server");
        let mut buf = Vec::new();
        let res = stream.read_to_end(&mut buf);

        //let mut f = File::create(path); // TODO use a different name path.
        Ok(())
    }

    pub fn progress_bar(buf: Buffer) {
        // TODO (Optional)
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
}